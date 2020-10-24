use crate::error;
use crate::file;
use crate::path_stuff;
use fs_extra;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

/// the Dir struct is a struct for helping you
/// working with directories
#[derive(Debug, Clone)]
pub struct Dir {
  /// the path of directory
  pub path: PathBuf,
}

// new and prop-like methods
#[allow(dead_code)]
impl Dir {
  /// creates a new Dir
  /// ```
  /// use fs_pro::Dir;
  ///
  /// let dir = Dir::new("/path/to/dir").unwrap();
  /// let dir = Dir::new(Path::new("/path/to/dir")).unwrap();
  /// let dir = Dir::new(PathBuf::from("/path/to/dir"));
  /// ```
  /// # Errors
  /// - given path is file
  /// - invalid path
  pub fn new<'a, P: 'a + AsRef<Path>>(path: P) -> error::Result<Dir> {
    let mut path_buf = PathBuf::new();
    path_buf.push(path);
    let real_path = path_buf.as_path();
    if real_path.exists() && real_path.is_file() {
      Err(error::Error::new_from_kind(error::ErrorKind::InvalidFolder))
    } else {
      Ok(Dir { path: path_buf })
    }
  }
  /// creates a directory in the temp directory
  /// ```
  /// use fs_pro::Dir;
  ///
  /// let temp_dir = Dir::temp_dir("name").unwrap();
  /// ```
  pub fn temp_dir<'a, P: 'a + AsRef<Path>>(name: P) -> error::Result<Dir> {
    let dir = Dir::temp_dir_no_create(name)?;
    dir.create()?;
    Ok(dir)
  }
  /// like `temp_dir` but doesn't create the directory
  /// ```
  /// use fs_pro::Dir;
  ///
  /// let temp_dir = Dir::temp_dir_no_create("name").unwrap();
  /// ```
  pub fn temp_dir_no_create<'a, P: 'a + AsRef<Path>>(name: P) -> error::Result<Dir> {
    let mut tmp_dir = std::env::temp_dir();
    tmp_dir.push(name);
    let dir = Dir::new(tmp_dir)?;
    Ok(dir)
  }
  /// create a directory in the temp directory with random name
  /// ```
  /// use fs_pro::Dir;
  ///
  /// let temp_dir = Dir::temp_dir_rand();
  /// ```
  pub fn temp_dir_rand() -> error::Result<Dir> {
    Ok(Dir::temp_dir(path_stuff::get_rand_chars(10))?)
  }
  /// like `temp_dir_rand` but doesn't create the directory
  /// ```
  /// use fs_pro::Dir;
  ///
  /// let temp_dir = Dir::temp_dir_rand_no_create();
  /// ```
  pub fn temp_dir_rand_no_create() -> error::Result<Dir> {
    Ok(Dir::temp_dir_no_create(path_stuff::get_rand_chars(10))?)
  }
  /// gets the parent of the directory in &str
  /// ```
  /// use fs_pro::Dir;
  ///
  /// let dir = Dir::temp_dir_rand().unwrap();
  /// assert_eq!(dir.parent().unwrap(), "/tmp");
  /// ```
  pub fn parent(&self) -> error::Result<&str> {
    path_stuff::parent(self.path.as_path())
  }
  /// gets the name of the directory in &str
  /// ```
  /// use fs_pro::Dir;
  ///
  /// let dir = Dir::temp_dir("my_dir").unwrap();
  /// assert_eq!(dir.name().unwrap(), "my_dir");
  /// ```
  pub fn name(&self) -> error::Result<&str> {
    path_stuff::name(self.path.as_path())
  }
  /// parses the path and returns fs_pro::ParedPathDir
  pub fn parse_path(&self) -> error::Result<path_stuff::ParsedPathDir<'_>> {
    path_stuff::parse_path_dir(self.path.as_path())
  }
  /// get the size of directory in bytes
  pub fn size(&self) -> error::Result<u64> {
    Ok(error::result_from_fse(fs_extra::dir::get_size(&self.path))?)
  }
}

#[allow(dead_code)]
impl Dir {
  /// return true if file exists
  pub fn exists(&self) -> bool {
    self.path.exists()
  }
  /// creates the directory if doesn't already exits
  pub fn create(&self) -> error::Result<()> {
    if self.path.exists() {
      Ok(())
    } else {
      error::result_from_io(fs::create_dir(&self.path))
    }
  }
  /// creates the directory and it's parent if doesn't exists
  pub fn create_all(&self) -> error::Result<()> {
    error::result_from_io(fs::create_dir_all(&self.path))?;
    Ok(())
  }
  /// delete the directory even if empty
  pub fn delete(&self) -> error::Result<()> {
    error::result_from_fse(fs_extra::dir::remove(&self.path))
  }
  /// create a file inside the directory and return fs_pro::File
  /// ```
  /// let file = dir.create_file("hi.txt")?;
  /// file.write("some thing")?;
  /// // ...
  /// ```
  pub fn create_file<P: AsRef<Path>>(&self, name: P) -> error::Result<file::File> {
    let mut file_path = PathBuf::new();
    file_path.push(&self.path);
    file_path.push(name);
    let file = file::File::new(file_path)?;
    file.create()?;
    Ok(file)
  }
  /// create a directory inside the directory and returns fs_pro::Dir
  /// ```
  /// let sub_dir = dir.create_dir("sub_dir")?;
  /// sub_dir.create_file("hello.txt")?;
  /// // ...
  /// ```
  pub fn create_dir<P: AsRef<Path>>(&self, name: P) -> error::Result<Dir> {
    let mut file_path = PathBuf::new();
    file_path.push(&self.path);
    file_path.push(name);
    let dir = Dir::new(file_path)?;
    dir.create()?;
    Ok(dir)
  }
  /// delete a file inside the directory
  /// ```
  /// dir.delete_file("to_delete.txt")?;
  /// ```
  pub fn delete_file<P: AsRef<Path>>(&self, name: P) -> error::Result<()> {
    let mut file_path = PathBuf::new();
    file_path.push(&self.path);
    file_path.push(name);
    error::result_from_io(fs::remove_file(file_path))
  }
  /// delete a directory inside the directory
  /// ```
  /// dir.delete_dir("sub_dir")?;
  /// ```
  pub fn delete_dir<P: AsRef<Path>>(&self, name: P) -> error::Result<()> {
    let mut file_path = PathBuf::new();
    file_path.push(&self.path);
    file_path.push(name);
    error::result_from_fse(fs_extra::dir::remove(file_path))
  }
  /// get a fs_pro::File inside the directory
  /// ```
  /// let file = dir.get_file("my_file.txt")?;
  /// file.create()?;
  /// // ...
  /// ```
  pub fn get_file<P: AsRef<Path>>(&self, name: P) -> error::Result<file::File> {
    let mut file_path = PathBuf::new();
    file_path.push(&self.path);
    file_path.push(name);
    Ok(file::File::new(file_path)?)
  }
  /// get a fs_pro::Dir inside the directory
  /// ```
  /// let sub_dir = dir.get_dir("sub_dir")?;
  /// sub_dir.create()?;
  /// // ...
  /// ```
  pub fn get_dir<P: AsRef<Path>>(&self, name: P) -> error::Result<Dir> {
    let mut file_path = PathBuf::new();
    file_path.push(&self.path);
    file_path.push(name);
    Ok(Dir::new(file_path)?)
  }
  /// copy the directory and returns directory's copy fs_pro::Dir
  /// ```
  /// let dir_copy = dir.copy("copy_path")?;
  /// dir_copy.create_file("some_file")?;
  /// // ...
  /// ```
  pub fn copy<P: AsRef<Path>>(
    &self,
    to: P,
    options: &fs_extra::dir::CopyOptions,
  ) -> error::Result<Dir> {
    let mut dest = PathBuf::new();
    dest.push(to);
    if !dest.exists() {
      error::result_from_io(fs::create_dir(&dest))?;
    }
    error::result_from_fse(fs_extra::dir::copy(&self.path, &dest, options))?;
    Ok(Dir::new(dest)?)
  }
  /// copy the directory with progress and returns directory's copy as fs_pro::Dir
  /// ```
  /// extern crate fs_extra;
  /// use fs_extra::dir::{CopyOptions, TransitProcess, TransitProcessResult};
  ///
  /// let options = CopyOptions::new();
  /// let handle = |process_info: TransitProcess|  {
  ///    println!("{}", process_info.total_bytes);
  ///    TransitProcessResult::ContinueOrAbort
  /// }
  /// let dir_copy = dir.copy_with_progress("dest_path", &options, handle)?;
  /// ```
  pub fn copy_with_progress<
    P: AsRef<Path>,
    F: FnMut(fs_extra::dir::TransitProcess) -> fs_extra::dir::TransitProcessResult,
  >(
    &self,
    to: P,
    options: &fs_extra::dir::CopyOptions,
    progress_handler: F,
  ) -> error::Result<Dir> {
    let mut dest = PathBuf::new();
    dest.push(to);
    if !dest.exists() {
      error::result_from_io(fs::create_dir(&dest))?;
    }
    error::result_from_fse(fs_extra::dir::copy_with_progress(
      &self.path,
      &dest,
      options,
      progress_handler,
    ))?;
    Ok(Dir::new(dest)?)
  }
  /// moves the directory and returns directory as fs_pro::Dir
  /// ```
  /// dir = dir.move("dest")?;
  /// dir.create_file("some_file")?;
  /// // ...
  /// ```
  pub fn move_to<P: AsRef<Path>>(
    &self,
    to: P,
    options: &fs_extra::dir::CopyOptions,
  ) -> error::Result<Dir> {
    let mut dest = PathBuf::new();
    dest.push(to);
    if !dest.exists() {
      error::result_from_io(fs::create_dir(&dest))?;
    }
    error::result_from_fse(fs_extra::dir::move_dir(&self.path, &dest, options))?;
    Ok(Dir::new(dest)?)
  }
  /// copy the directory with progress and returns directory's dest as fs_pro::Dir
  /// ```
  /// extern crate fs_extra;
  /// use fs_extra::dir::{CopyOptions, TransitProcess, TransitProcessResult};
  ///
  /// let options = CopyOptions::new();
  /// let handle = |process_info: TransitProcess|  {
  ///    println!("{}", process_info.total_bytes);
  ///    TransitProcessResult::ContinueOrAbort
  /// }
  /// dir = dir.move_with_progress("dest_path", &options, handle)?;
  /// ```
  pub fn move_to_with_progress<
    P: AsRef<Path>,
    F: FnMut(fs_extra::dir::TransitProcess) -> fs_extra::dir::TransitProcessResult,
  >(
    &self,
    to: P,
    options: &fs_extra::dir::CopyOptions,
    progress_handler: F,
  ) -> error::Result<Dir> {
    let mut dest = PathBuf::new();
    dest.push(to);
    if !dest.exists() {
      error::result_from_io(fs::create_dir(&dest))?;
    }
    error::result_from_fse(fs_extra::dir::move_dir_with_progress(
      &self.path,
      &dest,
      options,
      progress_handler,
    ))?;
    Ok(Dir::new(dest)?)
  }
  /// return fs_extra::dir::DirContent which contains information about directory
  /// see https://docs.rs/fs_extra/1.1.0/fs_extra/dir/fn.get_dir_content.html
  /// ```
  /// let dir_content = dir.get_content()?;
  /// for directory in dir_content.directories {
  ///   println!("{}", directory); // print directory path
  /// }
  /// ```
  pub fn get_content(&self) -> error::Result<fs_extra::dir::DirContent> {
    error::result_from_fse(fs_extra::dir::get_dir_content(&self.path))
  }
  /// return fs_extra::dir::DirContent which contains information about directory
  /// see https://docs.rs/fs_extra/1.1.0/fs_extra/dir/fn.get_dir_content2.html
  /// ```
  /// extern crate fs_extra;
  /// use fs_extra::dir::DirOptions;
  ///
  /// let options = DirOptions::new();
  /// options.depth = 3; // Get 3 levels of folder.
  /// let dir_content = get_dir_content2("dir", &options)?;
  /// for directory in dir_content.directories {
  ///    println!("{}", directory); // print directory path
  /// }
  /// ```  
  pub fn get_content2(
    &self,
    options: &fs_extra::dir::DirOptions,
  ) -> error::Result<fs_extra::dir::DirContent> {
    error::result_from_fse(fs_extra::dir::get_dir_content2(&self.path, options))
  }
  /// returns information about directory entry with information which you choose in config
  /// see https://docs.rs/fs_extra/1.1.0/fs_extra/dir/fn.get_details_entry.html
  /// ```
  /// extern crate fs_extra;
  /// use fs_extra::dir::{DirEntryAttr};
  /// use std::collections::{HashMap, HashSet};
  ///
  /// let mut config = HashSet::new();
  /// config.insert(DirEntryAttr::Name);
  /// config.insert(DirEntryAttr::Size);
  ///
  /// let entry_info = dir.get_details_entry(&config);
  /// assert_eq!(2, entry_info.len());
  /// ```
  pub fn get_details_entry(
    &self,
    config: &HashSet<fs_extra::dir::DirEntryAttr>,
  ) -> error::Result<HashMap<fs_extra::dir::DirEntryAttr, fs_extra::dir::DirEntryValue>> {
    error::result_from_fse(fs_extra::dir::get_details_entry(&self.path, config))
  }
  /// creates a directory inside directory and it's parent if missing
  /// ```
  /// let sub_dir = dir.create_dir_all("foo/bar/some") // creates foo and bar and some
  /// ```
  pub fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> error::Result<Dir> {
    let mut path_buf = PathBuf::new();
    path_buf.push(&self.path);
    path_buf.push(path);
    error::result_from_fse(fs_extra::dir::create_all(&path_buf, false))?;
    Ok(Dir::new(path_buf)?)
  }
  /// creates a file inside directory and it's parent if missing
  /// ```
  /// let file = dir.create_file_all("foo/bar/some.txt") // creates foo and bar and some.txt
  /// ```
  pub fn create_file_all<P: AsRef<Path>>(&self, path: P) -> error::Result<file::File> {
    let mut path_buf = PathBuf::new();
    path_buf.push(&self.path);
    path_buf.push(path);
    let path_buf_parent =
      error::result_from_option2(path_buf.parent(), error::ErrorKind::PathNoParentFound)?;
    error::result_from_fse(fs_extra::dir::create_all(path_buf_parent, false))?;
    let file = file::File::new(path_buf)?;
    file.create()?;
    Ok(file)
  }
  /// returns collection directory entries with information which you choose in config
  /// see https://docs.rs/fs_extra/1.1.0/fs_extra/dir/fn.ls.html
  /// ```
  /// extern crate fs_extra;
  /// use fs_extra::dir::DirEntryAttr;
  /// use std::collections::HashSet;
  /// let mut config = HashSet::new();
  /// config.insert(DirEntryAttr::Name);
  /// config.insert(DirEntryAttr::Size);
  /// config.insert(DirEntryAttr::BaseInfo);
  ///
  /// let result = dir.ls(&config);
  /// assert_eq!(2, ls_result.items.len());
  /// assert_eq!(2, ls_result.base.len());
  /// ```
  pub fn ls(
    &self,
    config: &HashSet<fs_extra::dir::DirEntryAttr>,
  ) -> error::Result<fs_extra::dir::LsResult> {
    error::result_from_fse(fs_extra::dir::ls(&self.path, config))
  }
}
