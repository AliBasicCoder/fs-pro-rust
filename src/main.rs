mod dir;
mod error;
mod file;
mod path_stuff;
use dir::Dir;
use file::File;
use fs_extra;

fn main() {
  #[cfg(feature = "json")]
  {
    let file10 = File::new("hi.json").unwrap();
    file10.write("{\"hi\":\"there\"}").unwrap();
    let json: serde_json::Value = file10.json().unwrap();
    println!("{:?}", json);
    file10.delete().unwrap();
  }
  println!("{:?}", File::temp_file_rand());
  let current_dir = std::env::current_dir().unwrap();
  println!("{:?}", current_dir);
  let file = File::new("hi.txt").unwrap();
  let parsed_path = file.parse_path().unwrap();
  println!("{:?}", parsed_path);
  println!("exists: {}", file.exists());
  // file.create().unwrap();
  file.write("hi there").unwrap();
  file.append(b"\nhello").unwrap();
  println!("===========\n{:?}", file.read().unwrap());
  println!("===========\n{}", file.read_to_string().unwrap());
  let mut file2 = file.copy("hgfh.txt").unwrap();
  let file3 = file
    .copy_with_progress("hfdghfd.txt", &fs_extra::file::CopyOptions::new(), |pi| {
      println!("{:?}", pi.total_bytes);
    })
    .unwrap();
  let mt = file2.metadata().unwrap();
  println!("file size: {}b", mt.len());
  file.delete().unwrap();
  file3.delete().unwrap();
  file2 = file2.move_to("hello.txt").unwrap();
  file2.delete().unwrap();
  // let mut perms = file2.metadata().unwrap().permissions();
  // perms.set_readonly(true);
  // file2.set_permissions(perms).unwrap();
  // println!("{:?}", path_stuff::join(&["/etc", "hello", "world"]))
  let dir_path = &path_stuff::join(&[current_dir.to_str().unwrap(), "hi there"]);
  let dir = Dir::new(dir_path).unwrap();
  println!("{:?}", dir.parse_path().unwrap());
  dir.create().unwrap();
  dir.create_file("hi.txt").unwrap();
  dir.create_dir("hi_again").unwrap();
  dir.create_dir_all("foo/bar").unwrap();
  dir.create_file_all("foo2/bar2/some.txt").unwrap();
  println!("{:?}", dir.get_file("hi.txt"));
  println!("{:?}", dir.get_file("foo2/bar2/some.txt"));
  println!("{:?}", dir.get_dir("hi_again"));
  println!("{:?}", dir.get_dir("foo/bar"));
  println!("{}", dir.size().unwrap());
  dir.delete_file("hi.txt").unwrap();
  dir.delete_dir("hi_again").unwrap();
  dir.delete().unwrap();
}
