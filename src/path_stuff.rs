use crate::error;
use rand::{self, Rng};
use std::path::Path;

pub fn get_rand_chars(len: usize) -> String {
  let mut rng = rand::thread_rng();
  let chars: String = std::iter::repeat(())
    .map(|()| rng.sample(rand::distributions::Alphanumeric))
    .take(len)
    .collect();
  chars
}

/// the result of fs_pro::File::parse_path
#[derive(Debug, Copy, Clone)]
pub struct ParsedPathFile<'a> {
  /// the path of file
  pub path: &'a str,
  // the parent of file
  pub parent: &'a str,
  /// the file name of file (including extension)
  pub name: &'a str,
  /// the file name of file (excluding extension)
  pub name_without_extension: &'a str,
  /// the extension of file
  pub extension: &'a str,
}

/// the result of fs_pro::Dir::parse_path
#[derive(Debug, Copy, Clone)]
pub struct ParsedPathDir<'a> {
  /// the path of directory
  pub path: &'a str,
  /// the parent of directory
  pub parent: &'a str,
  /// the name of directory
  pub name: &'a str,
}

#[macro_export]
macro_rules! join {
  ($($thing: expr),*) => {
     {
      use std::path::PathBuf;
      let mut path = PathBuf::new();
      $(path.push($thing);)*
      path
     }
  };
}

#[allow(dead_code)]
pub fn join<P: AsRef<Path>>(paths: &[P]) -> std::path::PathBuf {
  let mut path = std::path::PathBuf::new();
  for p in paths {
    path.push(p);
  }
  path
}

pub fn parent(path_inst: &Path) -> error::Result<&str> {
  let parent = error::result_from_option2(path_inst.parent(), error::ErrorKind::PathNoParentFound)?;
  let parent_to_str =
    error::result_from_option2(parent.to_str(), error::ErrorKind::PathToStrConversionFail)?;
  Ok(parent_to_str)
}

pub fn name(path_inst: &Path) -> error::Result<&str> {
  let name =
    error::result_from_option2(path_inst.file_name(), error::ErrorKind::PathNoFilenameFound)?;
  let name_to_str =
    error::result_from_option2(name.to_str(), error::ErrorKind::PathToStrConversionFail)?;
  Ok(name_to_str)
}

pub fn extension(path_inst: &Path) -> error::Result<&str> {
  let extension = error::result_from_option2(
    path_inst.extension(),
    error::ErrorKind::PathNoExtensionFound,
  )?;
  let extension_to_str = error::result_from_option2(
    extension.to_str(),
    error::ErrorKind::PathToStrConversionFail,
  )?;
  Ok(extension_to_str)
}

pub fn name_without_extension(path_inst: &Path) -> error::Result<&str> {
  let name = name(path_inst)?;
  if let Some(last_dot_index) = name.rfind(".") {
    Ok(&name[..last_dot_index])
  } else {
    Ok(name)
  }
}

pub fn path_to_str(path_inst: &Path) -> error::Result<&str> {
  Ok(error::result_from_option2(
    path_inst.to_str(),
    error::ErrorKind::PathToStrConversionFail,
  )?)
}

pub fn parse_path_file<'a>(path_inst: &'a Path) -> error::Result<ParsedPathFile<'a>> {
  let path = path_to_str(path_inst)?;
  let directory = parent(path_inst)?;
  let name = name(path_inst)?;
  let name_without_extension = name_without_extension(path_inst)?;
  let extension = match extension(path_inst) {
    Ok(val) => val,
    Err(_) => "",
  };
  Ok(ParsedPathFile {
    parent: directory,
    name: name,
    name_without_extension: name_without_extension,
    extension: extension,
    path: path,
  })
}

pub fn parse_path_dir<'a>(path_inst: &'a Path) -> error::Result<ParsedPathDir<'a>> {
  let path = path_to_str(path_inst)?;
  let directory = parent(path_inst)?;
  let name = name(path_inst)?;
  Ok(ParsedPathDir {
    parent: directory,
    name: name,
    path: path,
  })
}
