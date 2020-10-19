use fs_pro::{error, Dir, File, ParsedPathFile};
use std::{
  fs,
  path::{Path, PathBuf},
};

fn okay_to_err<T>(result: error::Result<T>) {
  match result {
    Ok(_) => {}
    Err(_) => {}
  }
}

#[test]
fn new() {
  // works with different path types
  File::new("/tmp/path/type").unwrap();
  File::new(Path::new("/tmp/path/type")).unwrap();
  File::new(PathBuf::from("/tmp/path/type")).unwrap();
  // errors if path is not file
  let dir = Dir::temp_dir_rand().unwrap();
  match File::new(&dir.path) {
    Ok(_) => {
      panic!("should pass");
    }
    Err(e) => match e.kind {
      error::ErrorKind::InvalidFile => {}
      _ => {
        panic!(format!("invalid error \"{}\"", e.message));
      }
    },
  };
  okay_to_err(dir.delete());
}

#[test]
fn temp_file() {
  let file = File::temp_file("hello_there").unwrap();
  let mut expected = std::env::temp_dir();
  expected.push("hello_there");
  assert_eq!(file.path, expected);
  okay_to_err(file.delete());
}

#[test]
fn temp_file_rand() {
  let file = File::temp_file_rand().unwrap();
  assert!(file.path.starts_with(std::env::temp_dir()));
  okay_to_err(file.delete());
}

#[test]
fn directory() {
  let file = File::temp_file_rand_no_create().unwrap();
  let temp_dir = std::env::temp_dir();
  let mut expected = String::from(temp_dir.to_str().unwrap());
  let dir = file.directory().unwrap();
  if expected.ends_with("/") || expected.ends_with("\\") {
    expected.pop();
  }
  assert_eq!(dir, expected.as_str());
  okay_to_err(file.delete());
}

#[test]
fn name() {
  // without extension
  let file = File::temp_file_no_create("hello_world").unwrap();
  assert_eq!(file.name().unwrap(), "hello_world");
  // with ext
  let file2 = File::temp_file_no_create("hello_world.txt").unwrap();
  assert_eq!(file2.name().unwrap(), "hello_world.txt");
}

#[test]
fn name_without_extension() {
  let file = File::temp_file_no_create("hello_world.txt").unwrap();
  assert_eq!(file.name_without_extension().unwrap(), "hello_world");
}

#[test]
fn extension() {
  let file = File::temp_file_no_create("hello_world.txt").unwrap();
  assert_eq!(file.extension().unwrap(), "txt");
}

#[test]
fn parse_path() {
  let file = File::temp_file_no_create("hello_world.txt").unwrap();
  let temp_dir = std::env::temp_dir();
  let mut temp_dir_str = String::from(temp_dir.to_str().unwrap());
  if temp_dir_str.ends_with("/") || temp_dir_str.ends_with("\\") {
    temp_dir_str.pop();
  }
  let parsed = file.parse_path().unwrap();
  let expected = ParsedPathFile {
    directory: temp_dir_str.as_str(),
    name: "hello_world.txt",
    extension: "txt",
    name_without_extension: "hello_world",
    path: file.path.to_str().unwrap(),
  };

  assert_eq!(parsed.directory, expected.directory);
  assert_eq!(parsed.name, expected.name);
  assert_eq!(
    parsed.name_without_extension,
    expected.name_without_extension
  );
  assert_eq!(parsed.extension, expected.extension);
  assert_eq!(parsed.path, expected.path);
}

#[test]
fn exists() {
  // when true
  let file = File::temp_file_rand().unwrap();
  assert_eq!(file.exists(), file.path.exists());
  // when false
  let file = File::temp_file_rand_no_create().unwrap();
  assert_eq!(file.exists(), file.path.exists());
  okay_to_err(file.delete());
}

#[test]
fn write() {
  let file = File::temp_file_rand_no_create().unwrap();
  file.write(b"hello world").unwrap();
  let actual = fs::read_to_string(&file.path).unwrap();
  assert_eq!(actual, "hello world");
  okay_to_err(file.delete());
}

#[test]
fn read() {
  let file = File::temp_file_rand().unwrap();
  file.write(b"hello world").unwrap();
  let actual = file.read().unwrap();
  assert_eq!(actual, b"hello world");
  okay_to_err(file.delete());
}

#[test]
fn read_to_string() {
  let file = File::temp_file_rand().unwrap();
  file.write(b"hello world").unwrap();
  let actual = file.read_to_string().unwrap();
  assert_eq!(actual, "hello world".to_string());
  okay_to_err(file.delete());
}

#[test]
fn append() {
  let file = File::temp_file_rand_no_create().unwrap();
  file.append("hello world").unwrap();
  let actual = file.read_to_string().unwrap();
  assert_eq!(actual, "hello world".to_string());
  okay_to_err(file.delete());
}

#[test]
fn delete() {
  let file = File::temp_file_rand().unwrap();
  file.delete().unwrap();
  assert_eq!(file.exists(), false);
}

#[test]
fn copy() {
  let file = File::temp_file_rand().unwrap();
  file.write("hello world").unwrap();
  let mut dest = std::env::temp_dir();
  dest.push("hello_there");
  let file_copy = file.copy(dest).unwrap();
  assert_eq!(file_copy.exists(), true);
  assert_eq!(
    file_copy.read_to_string().unwrap(),
    "hello world".to_string()
  );
  okay_to_err(file.delete());
  okay_to_err(file_copy.delete());
}

#[test]
fn move_to() {
  let file = File::temp_file_rand().unwrap();
  file.write("hello world").unwrap();
  let mut dest = std::env::temp_dir();
  dest.push("hello_there_2");
  let file_moved = file.move_to(dest).unwrap();
  assert_eq!(file_moved.exists(), true);
  assert_eq!(
    file_moved.read_to_string().unwrap(),
    "hello world".to_string()
  );
  assert_eq!(file.exists(), false);
  okay_to_err(file_moved.delete());
}

#[test]
fn rename() {
  let file = File::temp_file_rand().unwrap();
  let file_renamed = file.rename("new_name").unwrap();
  assert_eq!(file.exists(), false);
  assert_eq!(file_renamed.exists(), true);
  okay_to_err(file_renamed.delete());
}

#[test]
fn copy_with_progress() {
  let file = File::temp_file_rand().unwrap();
  file.write("hello world").unwrap();
  let mut dest = std::env::temp_dir();
  dest.push("hello_there_3");
  let mut called = 0;
  let file_copy = file
    .copy_with_progress(dest, &fs_extra::file::CopyOptions::new(), |_prg| {
      called += 1;
    })
    .unwrap();
  assert_eq!(file.exists(), true);
  assert_eq!(file_copy.exists(), true);
  assert!(called >= 1);
  okay_to_err(file.delete());
  okay_to_err(file_copy.delete());
}

#[test]
fn move_with_progress() {
  let file = File::temp_file_rand().unwrap();
  file.write("hello world").unwrap();
  let mut dest = std::env::temp_dir();
  dest.push("hello_there_4");
  let mut called = 0;
  let file_move = file
    .move_with_progress(dest, &fs_extra::file::CopyOptions::new(), |_prg| {
      called += 1;
    })
    .unwrap();
  assert_eq!(file.exists(), false);
  assert_eq!(file_move.exists(), true);
  assert!(called >= 1);
  okay_to_err(file.delete());
  okay_to_err(file_move.delete());
}
