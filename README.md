# fs pro

> A library to work with files easily

![crates.io](https://img.shields.io/crates/v/fs_pro.svg)
![Crates.io](https://img.shields.io/crates/d/fs_pro)

the is a beta rust version of [fs-pro](https://github.com/AliBasicCoder/fs-pro)

see the full docs [here](https://docs.rs/fs_pro)

## Features

- you don't have to work with std api's
- easy to use

## Usage

```rust
use fs_pro::{Dir, File, Shape, error::Result};

#[derive(Shape)]
struct ChildShapedDir {
  #[name = "child_file.txt"]
  child_file: File
  // ...
}

#[derive(Shape)]
struct MyShapedDir {
  #[name = "my_file.txt"]
  my_file: File,
  #[pattern = "*.txt"]
  my_dir: Dir,
  child_shaped_dir: ChildShapedDir
}


fn main() -> Result<()> {
  let file = File::new("my_file.txt");
  // create the file
  file.create();
  // write to file
  file.write("hello there");
  // read file
  file.read_to_string(); // => "hello there"
  // and much more...
  let dir = Dir::new("my_dir");
  // create the dir
  dir.create();
  // create a file in it
  dir.create_file("my_file.txt").unwrap().write("hello world");
  // create a dir in it
  dir.create_dir("my_dir");

  let shape: Shape<MyShapedDir> = Shape::new();
  let shape_inst = shape.create_at("target").unwrap();
  println!("{:?}", shape_inst.my_file); // File
  println!("{:?}", shape_inst.my_dir); // Dir
  println!("{:?}", shape_inst.child_shaped_dir.child_file); // File

  // and much more...
  Ok(())
}
```

## rust features

- json: adds method json on File

## Licence

Copyright (c) 2020 AliBasicCoder
