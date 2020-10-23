# fs pro

> A library to work with files easily

![crates.io](https://img.shields.io/crates/v/fs_pro.svg)
![Crates.io](https://img.shields.io/crates/d/fs_pro)

the is a beta rust version of [https://github.com/AliBasicCoder/fs-pro](fs-pro)

see the full docs [https://docs.rs/fs_pro](here)

## Features

- you don't have to work with std api's
- easy to use

## Usage

```rust
use fs_pro::{Dir, File, error::Result};

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
  // and much more...
}
```

## Licence

Copyright (c) 2020 AliBasicCoder
