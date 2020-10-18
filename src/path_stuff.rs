use std::path::Path;
#[derive(Debug, Copy, Clone)]
pub struct PathProp<'a> {
  path: &'a str,
  directory: &'a str,
  name: &'a str,
  name_without_extension: &'a str,
  extension: &'a str,
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
pub fn join(paths: &[&str]) -> std::path::PathBuf {
  let mut path = std::path::PathBuf::new();
  for p in paths {
    path.push(p);
  }
  path
}

pub fn directory(path_inst: &Path) -> Result<&str, &str> {
  if let Some(parent) = path_inst.parent() {
    if let Some(parent_to_str) = parent.to_str() {
      Ok(parent_to_str)
    } else {
      Err("failed to convert parent to &str")
    }
  } else {
    Err("no parent found")
  }
}
pub fn name(path_inst: &Path) -> Result<&str, &str> {
  if let Some(name) = path_inst.file_name() {
    if let Some(name_to_str) = name.to_str() {
      Ok(name_to_str)
    } else {
      Err("failed to convert name to &str")
    }
  } else {
    Err("file name not found")
  }
}
pub fn extension(path_inst: &Path) -> Result<&str, &str> {
  if let Some(extension) = path_inst.extension() {
    if let Some(extension_to_str) = extension.to_str() {
      Ok(extension_to_str)
    } else {
      Err("failed to convert extension to &str")
    }
  } else {
    Err("extension not found")
  }
}
pub fn name_without_extension(path_inst: &Path) -> Result<&str, &str> {
  match name(path_inst) {
    Ok(name) => {
      if let Some(last_dot_index) = name.rfind(".") {
        Ok(&name[..last_dot_index])
      } else {
        Ok(name)
      }
    }
    Err(e) => Err(e),
  }
}
pub fn path_as_path_prop<'a>(path_inst: &'a Path, path: &'a str) -> Result<PathProp<'a>, &'a str> {
  let directory = directory(path_inst)?;
  let name = name(path_inst)?;
  let extension = extension(path_inst)?;
  let name_without_extension = name_without_extension(path_inst)?;
  Ok(PathProp {
    directory: directory,
    name: name,
    name_without_extension: name_without_extension,
    extension: extension,
    path: path,
  })
}
