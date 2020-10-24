use crate::error;
use crate::path_stuff;
use fs_extra;
// use serde;
// use serde_json;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// the File struct is a struct to help you work with files
#[derive(Debug, Clone)]
pub struct File {
  /// the path of file
  pub path: PathBuf,
}

// new and static methods and prop-like methods
#[allow(dead_code)]
impl File {
  /// creates a new File
  /// ```
  /// use fs_pro::File;
  ///
  /// let path = File::new("/path/to/path").unwrap();
  /// let path = File::new(Path::new("/path/to/path")).unwrap();
  /// let path = File::new(PathBuf::from("/path/to/path"));
  /// ```
  /// # Errors
  /// - given path is directory
  /// - invalid path
  pub fn new<P: AsRef<Path>>(path: P) -> error::Result<File> {
    let mut path_buf = PathBuf::new();
    path_buf.push(path);
    let real_path = path_buf.as_path();
    if real_path.exists() && !real_path.is_file() {
      Err(error::Error::new(
        error::ErrorKind::InvalidFile,
        "path suppose to be file found folder",
      ))
    } else {
      Ok(File { path: path_buf })
    }
  }
  /// creates a file in the temp directory
  /// ```
  /// use fs_pro::File;
  /// 
  /// let temp_file = Fir::temp_file("name").unwrap();
  /// ```
  pub fn temp_file<P: AsRef<Path>>(name: P) -> error::Result<File> {
    let file = File::temp_file_no_create(name)?;
    file.create()?;
    Ok(file)
  }
  /// like `temp_file` but doesn't create file
  /// ```
  /// use fs_pro::File;
  /// 
  /// let temp_file = Fir::temp_file_no_create("name").unwrap();
  /// ```
  pub fn temp_file_no_create<P: AsRef<Path>>(name: P) -> error::Result<File> {
    let mut tmp_dir = std::env::temp_dir();
    tmp_dir.push(name);
    let file = File::new(tmp_dir)?;
    Ok(file)
  }
  /// create a file in the temp directory with random name
  /// ```
  /// use fs_pro::File;
  /// 
  /// let temp_file = File::temp_file_rand();
  /// ```
  pub fn temp_file_rand() -> error::Result<File> {
    Ok(File::temp_file(path_stuff::get_rand_chars(10))?)
  }
  /// like `temp_file_rand` but doesn't create file
  /// ```
  /// use fs_pro::File;
  /// 
  /// let temp_file = File::temp_file_rand_no_create();
  /// ```
  pub fn temp_file_rand_no_create() -> error::Result<File> {
    Ok(File::temp_file_no_create(path_stuff::get_rand_chars(10))?)
  }
  /// gets the parent of the file in &str
  /// ```
  /// use fs_pro::File;
  /// 
  /// let file = File::temp_file_rand().unwrap();
  /// assert_eq!(file.parent().unwrap(), "/tmp");
  /// ```
  pub fn parent(&self) -> error::Result<&str> {
    path_stuff::parent(self.path.as_path())
  }
  /// gets the file name (including extension) in &str
  /// ```
  /// use fs_pro::File
  /// 
  /// let file = File::new("my_file.txt").unwrap();
  /// assert_eq!(file.name().unwrap(), "my_file.txt");
  /// ```
  pub fn name(&self) -> error::Result<&str> {
    path_stuff::name(self.path.as_path())
  }
  /// gets the file name (excluding extension) in &str
  /// ```
  /// use fs_pro::File
  /// 
  /// let file = File::new("my_file.txt").unwrap();
  /// assert_eq!(file.name_without_extension().unwrap(), "my_file");
  /// ```
  pub fn name_without_extension(&self) -> error::Result<&str> {
    path_stuff::name_without_extension(self.path.as_path())
  }
  /// gets the extension of file in &str
  /// ```
  /// use fs_pro::File
  /// 
  /// let file = File::new("my_file.txt").unwrap();
  /// assert_eq!(file.extension().unwrap(), "txt");
  /// ```
  pub fn extension(&self) -> error::Result<&str> {
    path_stuff::extension(self.path.as_path())
  }
  /// parses the file path and returns fs_pro::ParsedPathFile
  pub fn parse_path(&self) -> error::Result<path_stuff::ParsedPathFile<'_>> {
    path_stuff::parse_path_file(self.path.as_path())
  }
  /// returns the size of file in bytes
  pub fn size(&self) -> error::Result<u64> {
    Ok(self.metadata()?.len())
  }
}

#[allow(dead_code)]
impl File {
  /// return true if the file exists
  pub fn exists(&self) -> bool {
    self.path.exists()
  }
  /// create the file if it doesn't exists
  pub fn create(&self) -> error::Result<()> {
    if !self.exists() {
      error::result_from_io(fs::File::create(self.path.as_path()))?;
      Ok(())
    } else {
      Ok(())
    }
  }
  /// creates the directory and it's parent if doesn't exists
  pub fn create_all(&self) -> error::Result<()> {
    let parent = error::result_from_option2(self.path.parent(), error::ErrorKind::PathNoParentFound)?;
    error::result_from_io(fs::create_dir_all(parent))?;
    self.create()?;
    Ok(())
  }
  /// writes to file
  /// ```
  /// file.write("hi");
  /// file.write(vec![10, 100, 100]);
  /// ```
  pub fn write<C: AsRef<[u8]>>(&self, content: C) -> error::Result<()> {
    error::result_from_io(fs::write(self.path.as_path(), content))
  }
  /// reads the file as Vec<u8>
  /// ```
  /// file.read() // => [10, 124, ...]
  /// ```
  pub fn read(&self) -> error::Result<Vec<u8>> {
    error::result_from_io(fs::read(self.path.as_path()))
  }
  /// reads the file as String
  /// ```
  /// file.read_to_string() // => "hello"
  /// ```
  pub fn read_to_string(&self) -> error::Result<String> {
    error::result_from_io(fs::read_to_string(self.path.as_path()))
  }
  /// append to file
  /// NOTE: will create file if it doesn't exists
  /// ```
  /// file.append(b"hello world");
  /// file.append("hello world");
  /// file.append("hello world".to_string());
  /// ```
  pub fn append<C: AsRef<[u8]>>(&self, content: C) -> error::Result<()> {
    let maybe_file = fs::OpenOptions::new()
      .write(true)
      .append(true)
      .create(true)
      .open(self.path.as_path());
    let mut file = error::result_from_io(maybe_file)?;
    error::result_from_io(file.write_all(content.as_ref()))
  }
  /// get the metadata of the file
  ///
  /// see https://doc.rust-lang.org/std/fs/struct.Metadata.html
  pub fn metadata(&self) -> error::Result<fs::Metadata> {
    error::result_from_io(fs::metadata(self.path.as_path()))
  }
  /// deletes the file
  pub fn delete(&self) -> error::Result<()> {
    error::result_from_io(fs::remove_file(self.path.as_path()))
  }
  /// copies the file to dest
  /// ```
  /// let file_copy = file.copy("dest.txt");
  /// ```
  pub fn copy<P: AsRef<Path>>(&self, destination: P) -> error::Result<File> {
    let mut dest = PathBuf::new();
    dest.push(destination);
    error::result_from_io(fs::copy(&self.path.as_path(), &dest))?;
    Ok(File::new(dest)?)
  }
  /// move the file to dest
  /// ```
  /// let file_moved = file.move_to("dest");
  /// // or if you prefer
  /// file = file.move_to("dest");
  /// ```
  pub fn move_to<P: AsRef<Path>>(&self, destination: P) -> error::Result<File> {
    let options = fs_extra::file::CopyOptions::new();
    let mut dest = PathBuf::new();
    dest.push(destination);
    error::result_from_fse(fs_extra::file::move_file(
      self.path.as_path(),
      &dest,
      &options,
    ))?;
    Ok(File::new(dest)?)
  }
  /// renames the file
  /// NOTE: DO NOT use absolute paths with this function (use moveTo instead)
  /// ```
  /// let file_renamed = file.rename("new_name.txt");
  /// // or if you prefer
  /// file = file.rename("new_name.txt");
  /// ```
  pub fn rename<P: AsRef<Path>>(&self, dest: P) -> error::Result<File> {
    let mut real_dest = PathBuf::new();
    let parent =
      error::result_from_option2(self.path.parent(), error::ErrorKind::PathNoParentFound)?;
    real_dest.push(parent);
    real_dest.push(dest);
    self.move_to(real_dest)
  }
  /// sets the permissions of file see https://doc.rust-lang.org/std/fs/struct.Permissions.html
  /// ```
  /// let mut perm = file.metadata()?.permissions();
  /// perms.set_readonly(true);
  /// file.set_permissions(perms)?;
  /// ```
  pub fn set_permissions(&self, perm: fs::Permissions) -> error::Result<()> {
    let file = error::result_from_io(fs::File::open(self.path.as_path()))?;
    error::result_from_io(file.set_permissions(perm))
  }
  /// copy the file with progress
  /// ```
  /// use fs_extra;
  /// file.copy_with_progress("dest", &fs_extra::file::CopyOptions::new(), |prg| {
  ///   println!("{:?}", prg.total_bytes);
  /// })
  /// ```
  pub fn copy_with_progress<F: FnMut(fs_extra::file::TransitProcess), P: AsRef<Path>>(
    &self,
    to: P,
    options: &fs_extra::file::CopyOptions,
    progress_handler: F,
  ) -> error::Result<File> {
    error::result_from_fse(fs_extra::file::copy_with_progress(
      self.path.as_path(),
      &to,
      &options,
      progress_handler,
    ))?;
    Ok(File::new(to)?)
  }
  /// move the file with progress
  /// ```
  /// use fs_extra;
  /// file.move_file_with_progress("dest", &fs_extra::file::CopyOptions::new(), |prg| {
  ///   println!("{:?}", prg.total_bytes);
  /// })
  /// ```
  pub fn move_with_progress<F: FnMut(fs_extra::file::TransitProcess), P: AsRef<Path>>(
    &self,
    to: P,
    options: &fs_extra::file::CopyOptions,
    progress_handler: F,
  ) -> error::Result<File>
  where
    F: FnMut(fs_extra::file::TransitProcess),
  {
    error::result_from_fse(fs_extra::file::move_file_with_progress(
      self.path.as_path(),
      &to,
      &options,
      progress_handler,
    ))?;
    Ok(File::new(to)?)
  }
  /// parses file as json
  /// ```
  /// use serde_json::Value;
  /// 
  /// let json: Value = file.json();
  /// ```
  #[cfg(feature = "json")]
  pub fn json<T: for<'de> serde::Deserialize<'de>>(&self) -> error::Result<T> {
    let file = error::result_from_io(fs::File::open(&self.path))?;
    let reader = std::io::BufReader::new(file);
    let maybe_res: serde_json::error::Result<T> = serde_json::from_reader(reader);
    match maybe_res {
      Ok(res) => Ok(res),
      Err(e) => Err(error::Error::new_from_kind(error::ErrorKind::JsonError(e)))
    }
  }
}
