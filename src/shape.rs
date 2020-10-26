#![allow(dead_code)]
use crate::error;
use crate::{dir::Dir, file::File};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[derive(PartialEq)]
#[allow(missing_docs)]
pub enum ShapeItemStatic<'a> {
  File(&'a str, &'a str),
  DirectoryPattern(&'a str, &'a str, &'a str),
  DirectorySchema(&'a str, &'a str, &'a ShapeSchemaStatic<'a>),
}

impl ShapeItemStatic<'_> {
  fn identifier(&self) -> &&str {
    match self {
      ShapeItemStatic::File(identifier, _)
      | ShapeItemStatic::DirectoryPattern(identifier, _, _)
      | ShapeItemStatic::DirectorySchema(identifier, _, _) => identifier,
    }
  }
}

#[allow(missing_docs)]
pub type ShapeSchemaStatic<'a> = [ShapeItemStatic<'a>];

#[derive(Debug)]
#[allow(missing_docs)]
pub enum ShapeInstItem {
  File(File),
  Directory(Dir),
  ShapedDirectory(ShapeInst),
}

#[allow(missing_docs)]
pub type ShapeInst = Vec<ShapeInstItem>;

#[allow(missing_docs)]
pub trait ShapeDescribe {
  fn shape_describe() -> &'static ShapeSchemaStatic<'static>;
  fn shape_new(inst: ShapeInst) -> Self;
}

#[allow(missing_docs)]
pub trait ShapeDescribeStatic {
  fn shape_describe() -> &'static ShapeSchemaStatic<'static>;
  fn shape_new(inst: ShapeInst) -> Self;
}

/// Shape is a struct used to create directory with a specified Shape
///
/// in the example below we are defining shaped like so
///
/// the directory will contain
/// - a file named "my_file.txt" and it's identifier
/// (the named that we will access it with in our code) will be "my_file"
/// - a directory called "my_dir"
/// - a directory that will contain
///   - a file named "child_file.txt" and it's identifier will be "child_file"
///
/// example:
/// ```
/// use fs_pro::{File, Dir, Shape};
///
/// #[derive(Shape)]
/// struct ChildShapedDir {
///   #[name = "child_file.txt"]
///   child_file: File
///   /// ...
/// }
///
/// #[derive(Shape)]
/// struct MyShapedDir {
///   #[name = "my_file.txt"]
///   my_file: File,
///   #[pattern = "*.txt"]
///   my_dir: Dir,
///   child_shaped_dir: ChildShapedDir
/// }
///
/// fn main() {
///   let shape: Shape<MyShapedDir> = Shape::new();
///   let shape_inst = shape.create_at("target").unwrap();
///   println!("{:?}", shape_inst.my_file); // File
///   println!("{:?}", shape_inst.my_dir); // Dir
///   println!("{:?}", shape_inst.child_shaped_dir.child_file); // File
/// }
/// ```
pub struct Shape<T: ShapeDescribe> {
  /// this filed is there to avoid E0392 error
  #[allow(dead_code)]
  ignore_me: Option<T>,
}

impl<T: ShapeDescribe> Shape<T> {
  /// creates a new shape
  pub fn new() -> Self {
    Self { ignore_me: None }
  }
  /// create the shape in a directory
  pub fn create_at<'a, P: 'a + AsRef<Path>>(&self, path: P) -> Result<T, error::Error> {
    let target = T::shape_describe();
    let mut path_buf = PathBuf::new();
    path_buf.push(path);
    let res = create_shape_inst(path_buf, target, true, None)?;
    Ok(T::shape_new(res))
  }
  ///
  pub fn create_at_hook<'a, P: 'a + AsRef<Path>>(
    &self,
    path: P,
    hook: &'a dyn Fn(PathBuf, bool),
  ) -> Result<T, error::Error> {
    let target = T::shape_describe();
    let mut path_buf = PathBuf::new();
    path_buf.push(path);
    let res = create_shape_inst(path_buf, target, true, Some(hook))?;
    Ok(T::shape_new(res))
  }
  /// checks if a folder matches the shape and returns list of errors if they don't match
  /// the errors specify why and where they are not matching
  /// # Meaning of Errors
  /// - NotFound         if any file or folder doesn't exist
  /// - Interrupted      if operation interrupts (aka been stooped)
  /// - InvalidFile      if a file that doesn't match the pattern in a pattern dir was found
  /// - InvalidFolder    if a folder was found in a pattern dir
  /// - PermissionDenied if the os refuses to give the program permission to read form disk
  /// any other error that's not listed is IMPOSSIBLE to occur
  pub fn validate<'a, P: 'a + AsRef<Path>>(&self, path: P) -> Result<(), Errors> {
    let mut errors: Errors = vec![];
    let dir = Dir::new(path);
    if let Err(e) = dir {
      errors.push(e);
      return Err(errors);
    }
    let dir = dir.unwrap();
    let target = T::shape_describe();
    let errors = validate_dir(dir, target);
    if errors.len() > 0 {
      return Err(errors);
    }
    Ok(())
  }
}

fn check_pattern(pattern: &&str, path: OsString) -> bool {
  let pattern = pattern.replace(".", "\\.").replace("*", ".*");
  let regex = regex::Regex::new(pattern.as_str()).unwrap();
  if !regex.is_match(path.to_string_lossy().as_ref()) {
    return false;
  }
  true
}

use error::{Error, ErrorKind};

type Errors = Vec<Error>;

fn validate_dir(dir: Dir, target: &'static ShapeSchemaStatic<'static>) -> Errors {
  // TODO implement __rest
  let mut errors: Errors = vec![];
  if !dir.exists() {
    errors.push(Error::new_from_kind(ErrorKind::NotFound).set_path(dir.path.clone()));
    return errors;
  }
  for item in target {
    match item {
      ShapeItemStatic::File(_, name) => {
        if !dir.entry_exists(name) {
          errors.push(Error::new_from_kind(ErrorKind::NotFound).set_path(dir.path.join(name)));
        }
      }
      ShapeItemStatic::DirectoryPattern(_, name, pattern) => {
        if !dir.entry_exists(name) {
          errors.push(Error::new_from_kind(ErrorKind::NotFound).set_path(dir.path.join(name)));
          continue;
        }
        let sub_dir = dir.get_dir(name);
        if let Err(e) = sub_dir {
          errors.push(e);
          continue;
        }
        let sub_dir = sub_dir.unwrap();
        let sub_dir_read = sub_dir.read();
        if let Err(e) = sub_dir_read {
          errors.push(e);
          continue;
        }
        let sub_dir_read = sub_dir_read.unwrap();
        for entry in sub_dir_read {
          if !entry.path().is_file() {
            errors.push(Error::new_from_kind(ErrorKind::InvalidFolder).set_path(entry.path()));
            continue;
          }
          if !check_pattern(pattern, entry.file_name()) {
            errors.push(
              Error::new2(
                ErrorKind::InvalidFile,
                format!("file doesn't match pattern \"{}\"", pattern),
              )
              .set_path(entry.path()),
            );
            continue;
          }
        }
      }
      ShapeItemStatic::DirectorySchema(_, name, schema) => {
        if !dir.entry_exists(name) {
          errors.push(Error::new_from_kind(ErrorKind::NotFound).set_path(dir.path.join(name)));
          continue;
        }
        let dir_get_dir = dir.get_dir(name);
        if let Err(e) = dir_get_dir {
          errors.push(e);
          continue;
        }
        let dir_get_dir = dir_get_dir.unwrap();
        errors.append(&mut validate_dir(dir_get_dir, schema));
      }
    }
  }
  errors
}

fn create_shape_inst(
  path_buf: PathBuf,
  target_shape: &[ShapeItemStatic<'_>],
  create: bool,
  hook: Option<&dyn Fn(PathBuf, bool)>,
) -> Result<ShapeInst, error::Error> {
  let mut res: ShapeInst = vec![];
  for shape_item in target_shape {
    match shape_item {
      ShapeItemStatic::File(_, name) => {
        let file = File::new(path_buf.join(name))?;
        if create {
          file.create()?;
          if let Some(hook_fn) = hook {
            hook_fn(path_buf.join(name), true);
          }
        }
        res.push(ShapeInstItem::File(file));
      }
      ShapeItemStatic::DirectoryPattern(_, name, _) => {
        let dir = Dir::new(path_buf.join(name))?;
        if create {
          dir.create()?;
          if let Some(hook_fn) = hook {
            hook_fn(path_buf.join(name), true);
          }
        }
        res.push(ShapeInstItem::Directory(dir));
      }
      ShapeItemStatic::DirectorySchema(_, name, schema) => {
        let dir = Dir::new(path_buf.join(name))?;
        if create {
          dir.create()?;
          if let Some(hook_fn) = hook {
            hook_fn(path_buf.join(name), true);
          }
        }
        let child = create_shape_inst(path_buf.join(name), schema, create, hook)?;
        res.push(ShapeInstItem::ShapedDirectory(child));
      }
    }
  }
  Ok(res)
}

impl std::fmt::Debug for ShapeItemStatic<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ShapeItemStatic::File(_, name) => write!(f, "File('{}')", name),
      ShapeItemStatic::DirectoryPattern(_, name, pattern) => {
        write!(f, "Dir('{}', FilePattern({}))", name, pattern)
      }
      ShapeItemStatic::DirectorySchema(_, name, inner_schema) => {
        write!(f, "Dir('{}',", name)?;
        if inner_schema.len() > 0 {
          let mut fo = f.debug_struct("");
          for schema in *inner_schema {
            fo.field(schema.identifier(), schema);
          }
          fo.finish()?;
        } else {
          write!(f, " {{}}")?;
        }
        write!(f, ")")
      }
    }
  }
}

impl<T: ShapeDescribe> std::fmt::Debug for Shape<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut formatter = f.debug_struct("Shape");
    for schema in T::shape_describe() {
      formatter.field(schema.identifier(), &schema);
    }
    formatter.finish()
  }
}
