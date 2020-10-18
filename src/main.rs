mod file;
mod path_stuff;
use file::File;

fn main() {
    println!("{:?}", std::env::current_dir().unwrap());
    let file = File::new("hi.txt").unwrap();
    let path_prop = file.path_as_path_prop().unwrap();
    println!("{:?}", path_prop);
    println!("exists: {}", file.exists());
    // file.create().unwrap();
    file.write("hi there").unwrap();
    file.append(b"\nhello").unwrap();
    file.append_str("\nhello again").unwrap();
    file.append_string(&"\nhello again".to_string()).unwrap();
    println!("===========\n{:?}", file.read().unwrap());
    println!("===========\n{}", file.read_to_string().unwrap());
    let mut file2 = file.copy("hgfh.txt").unwrap();
    file.delete().unwrap();
    file2 = file2.move_to("hello.txt").unwrap();
    file2.delete().unwrap();
    println!("{:?}", path_stuff::join(&["/etc", "hello", "world"]))
}
