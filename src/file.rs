use crate::path_stuff;
use fs_extra;
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Copy, Clone)]
pub struct File<'a> {
  path: &'a str,
  path_inst: &'a Path,
}

// new and prop methods
#[allow(dead_code)]
impl File<'_> {
  pub fn new(path_string: &str) -> Result<File, &str> {
    let path_inst = Path::new(path_string);
    if path_inst.exists() && !path_inst.is_file() {
      Err("path suppose to be file")
    } else {
      Ok(File {
        path: path_inst.to_str().unwrap(),
        path_inst: path_inst,
      })
    }
  }
  pub fn directory(&self) -> Result<&str, &str> {
    path_stuff::directory(self.path_inst)
  }
  pub fn name(&self) -> Result<&str, &str> {
    path_stuff::name(self.path_inst)
  }
  pub fn name_without_extension(&self) -> Result<&str, &str> {
    path_stuff::name_without_extension(self.path_inst)
  }
  pub fn extension(&self) -> Result<&str, &str> {
    path_stuff::extension(self.path_inst)
  }
  pub fn path_as_path_prop(&self) -> Result<path_stuff::PathProp, &str> {
    path_stuff::path_as_path_prop(self.path_inst, self.path)
  }
}

#[allow(dead_code)]
impl File<'_> {
  /// return true if the file exists
  pub fn exists(&self) -> bool {
    self.path_inst.exists()
  }
  /// create the file if it doesn't exists
  pub fn create(&self) -> Result<(), std::io::Error> {
    if !self.exists() {
      std::fs::File::create(self.path_inst)?;
      Ok(())
    } else {
      Ok(())
    }
  }
  /// writes to file
  /// ```
  /// file.write("hi");
  /// file.write(vec![10, 100, 100]);
  /// ```
  pub fn write<C: AsRef<[u8]>>(&self, content: C) -> std::io::Result<()> {
    fs::write(self.path, content)
  }
  /// reads the file as Vec<u8>
  /// ```
  /// file.read() // => [10, 124, ...]
  /// ```
  pub fn read(&self) -> std::io::Result<Vec<u8>> {
    fs::read(self.path)
  }
  /// reads the file as String
  /// ```
  /// file.read_to_string() // => "hello"
  /// ```
  pub fn read_to_string(&self) -> std::io::Result<String> {
    fs::read_to_string(self.path)
  }
  /// append a &str to file
  /// NOTE: will create file if it doesn't exists
  /// ```
  /// file.append_str("hi there")
  /// ```
  pub fn append_str(&self, content: &str) -> std::io::Result<()> {
    self.append(content.as_bytes())
  }
  /// append a String to file
  /// NOTE: will create file if it doesn't exists
  /// ```
  /// file.append_string(String::from("hi there"))
  /// ```
  pub fn append_string(&self, content: &String) -> std::io::Result<()> {
    self.append(content.as_bytes())
  }
  /// append to file
  /// NOTE: will create file if it doesn't exists
  /// ```
  /// file.append(b"hello world");
  /// ```
  pub fn append(&self, content: &[u8]) -> std::io::Result<()> {
    let mut file = fs::OpenOptions::new()
      .write(true)
      .append(true)
      .create(true)
      .open(self.path)?;
    file.write_all(content)
  }
  /// get the metadata of the file
  ///
  /// see https://doc.rust-lang.org/std/fs/struct.Metadata.html
  pub fn metadata(&self) -> std::io::Result<fs::Metadata> {
    fs::metadata(self.path)
  }
  /// deletes the file
  pub fn delete(&self) -> std::io::Result<()> {
    fs::remove_file(self.path)
  }
  /// copies the file to dest
  /// ```
  /// let file_copy = file.copy("dest.txt");
  /// ```
  pub fn copy<'a>(&'a self, dest: &'a str) -> std::io::Result<File> {
    fs::copy(self.path, dest)?;
    match File::new(dest) {
      Ok(file) => Ok(file),
      Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
    }
  }
  /// move the file to dest
  /// ```
  /// let file_moved = file.move_to("dest");
  /// // or if you prefer
  /// file = file.move_to("dest");
  pub fn move_to(self, dest: &str) -> fs_extra::error::Result<File> {
    let options = fs_extra::file::CopyOptions::new();
    let dest_path = Path::new(dest);
    fs_extra::file::move_file(self.path_inst, dest_path, &options)?;
    match File::new(dest_path.to_str().unwrap()) {
      Ok(file) => Ok(file),
      Err(e) => Err(fs_extra::error::Error::new(
        fs_extra::error::ErrorKind::Other,
        e,
      )),
    }
  }
  /// renames the file
  /// ```
  /// let file_renamed = file.rename("new_name.txt");
  /// // or if you prefer
  /// file = file.rename("new_name.txt");
  pub fn rename(self, dest: &str) -> fs_extra::error::Result<File> {
    self.move_to(dest)
  }
}
