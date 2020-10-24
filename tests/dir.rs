use fs_pro::{error, Dir, File, ParsedPathDir};

fn okay_to_err<T, E>(result: Result<T, E>) {
  match result {
    Ok(_) => {}
    Err(_) => {}
  }
}

#[test]
fn new() -> error::Result<()> {
  let file = File::temp_file_rand()?;
  match Dir::new(file.path) {
    Ok(_) => {
      return Err(error::Error::new(
        error::ErrorKind::Other,
        "should not allow file paths",
      ));
    }
    Err(_) => {}
  };
  Ok(())
}

#[test]
fn temp_dir() -> error::Result<()> {
  let dir = Dir::temp_dir("hi there")?;
  let mut expected = std::env::temp_dir();
  expected.push("hi there");
  assert_eq!(dir.path, expected);
  assert_eq!(dir.path.exists(), true);
  okay_to_err(dir.delete());
  Ok(())
}

#[test]
fn temp_dir_rand() -> error::Result<()> {
  let dir = Dir::temp_dir_rand()?;
  let expected = std::env::temp_dir();
  assert!(dir.path.starts_with(expected));
  assert_eq!(dir.path.exists(), true);
  okay_to_err(dir.delete());
  Ok(())
}

#[test]
fn temp_dir_no_create() -> error::Result<()> {
  let dir = Dir::temp_dir_no_create("hi there2")?;
  let mut expected = std::env::temp_dir();
  expected.push("hi there2");
  assert_eq!(dir.path, expected);
  assert_eq!(dir.path.exists(), false);
  okay_to_err(dir.delete());
  Ok(())
}

#[test]
fn temp_dir_rand_no_create() -> error::Result<()> {
  let dir = Dir::temp_dir_rand_no_create()?;
  let expected = std::env::temp_dir();
  assert!(dir.path.starts_with(expected));
  assert_eq!(dir.path.exists(), false);
  okay_to_err(dir.delete());
  Ok(())
}

#[test]
fn parent() -> error::Result<()> {
  let dir = Dir::temp_dir_rand_no_create()?;
  let temp_dir = std::env::temp_dir();
  let mut expected = String::from(temp_dir.to_str().unwrap());
  if expected.ends_with("/") || expected.ends_with("\\") {
    expected.pop();
  }
  assert_eq!(dir.parent()?, expected.as_str());
  Ok(())
}

#[test]
fn name() -> error::Result<()> {
  let dir = Dir::temp_dir_no_create("hello there3")?;
  assert_eq!(dir.name()?, "hello there3");
  Ok(())
}

#[test]
fn parse_path() -> error::Result<()> {
  let dir = Dir::temp_dir_no_create("hello there4")?;
  let parsed = dir.parse_path()?;
  let temp_dir = std::env::temp_dir();
  let mut temp_dir_str = String::from(temp_dir.to_str().unwrap());
  if temp_dir_str.ends_with("/") || temp_dir_str.ends_with("\\") {
    temp_dir_str.pop();
  }
  let expected = ParsedPathDir {
    path: dir.path.to_str().unwrap(),
    parent: temp_dir_str.as_str(),
    name: "hello there4",
  };
  assert_eq!(parsed.parent, expected.parent);
  assert_eq!(parsed.name, expected.name);
  assert_eq!(parsed.path, expected.path);
  Ok(())
}

#[test]
fn create() -> error::Result<()> {
  let dir = Dir::temp_dir_rand_no_create()?;
  dir.create()?;
  assert_eq!(dir.path.exists(), true);
  okay_to_err(dir.delete());
  Ok(())
}

#[test]
fn create_all() -> error::Result<()> {
  let dir = Dir::temp_dir_no_create("foo/bar").unwrap();
  dir.create_all()?;
  assert_eq!(dir.path.exists(), true);
  okay_to_err(fs_extra::dir::remove(
    dir.path.parent().unwrap().parent().unwrap(),
  ));
  Ok(())
}

// tests create_file get_file delete_file
#[test]
fn _file() -> error::Result<()> {
  let dir = Dir::temp_dir_rand()?;
  let file = dir.create_file("some.txt")?;
  let file2 = dir.get_file("some.txt")?;
  assert_eq!(file.exists(), true);
  assert_eq!(file2.exists(), true);
  dir.delete_file("some.txt")?;
  assert_eq!(file.exists(), false);
  okay_to_err(dir.delete());
  Ok(())
}

// tests create_dir get_dir delete_dir
#[test]
fn _dir() -> error::Result<()> {
  let dir = Dir::temp_dir_rand()?;
  let sub_dir = dir.create_dir("sub_dir")?;
  let sub_dir2 = dir.get_dir("sub_dir")?;
  assert_eq!(sub_dir.exists(), true);
  assert_eq!(sub_dir2.exists(), true);
  dir.delete_dir("sub_dir")?;
  assert_eq!(sub_dir.exists(), false);
  okay_to_err(dir.delete());
  Ok(())
}

#[test]
fn copy() -> error::Result<()> {
  let dir = Dir::temp_dir_rand()?;
  let mut dest = std::env::temp_dir();
  dest.push("hi there6");
  let dir_copy = dir.copy(&dest, &fs_extra::dir::CopyOptions::new())?;
  assert_eq!(dir_copy.path, dest);
  assert_eq!(dir_copy.exists(), true);
  okay_to_err(dir.delete());
  okay_to_err(dir_copy.delete());
  Ok(())
}

#[test]
fn move_to() -> error::Result<()> {
  let dir = Dir::temp_dir_rand()?;
  let mut dest = std::env::temp_dir();
  dest.push("hi there7");
  let dir_move = dir.move_to(&dest, &fs_extra::dir::CopyOptions::new())?;
  assert_eq!(dir_move.path, dest);
  assert_eq!(dir_move.exists(), true);
  assert_eq!(dir.exists(), false);
  okay_to_err(dir_move.delete());
  Ok(())
}

#[test]
fn copy_with_progress() -> error::Result<()> {
  let dir = Dir::temp_dir_rand()?;
  dir.create_file("hi.txt")?.write("hello world")?;
  let mut dest = std::env::temp_dir();
  dest.push("hi there8");
  let mut called = 0;
  let dir_copy = dir.copy_with_progress(&dest, &fs_extra::dir::CopyOptions::new(), |_prg| {
    called += 1;
    fs_extra::dir::TransitProcessResult::ContinueOrAbort
  })?;
  assert!(called >= 1);
  assert_eq!(dir_copy.path, dest);
  assert_eq!(dir_copy.exists(), true);
  okay_to_err(dir.delete());
  okay_to_err(dir_copy.delete());
  Ok(())
}

#[test]
fn move_to_with_progress() -> error::Result<()> {
  let dir = Dir::temp_dir_rand()?;
  dir.create_file("hi.txt")?.write("hello world")?;
  let mut dest = std::env::temp_dir();
  dest.push("hi there9");
  let mut called = 0;
  let dir_move = dir.move_to_with_progress(&dest, &fs_extra::dir::CopyOptions::new(), |_prg| {
    called += 1;
    fs_extra::dir::TransitProcessResult::ContinueOrAbort
  })?;
  assert!(called >= 1);
  assert_eq!(dir_move.path, dest);
  assert_eq!(dir_move.exists(), true);
  assert_eq!(dir.exists(), false);
  okay_to_err(dir_move.delete());
  Ok(())
}
