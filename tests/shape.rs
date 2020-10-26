use fs_pro::{error, Dir, File, Shape};

fn okay_to_err<T, E>(result: Result<T, E>) {
  match result {
    Ok(_) => {}
    Err(_) => {}
  }
}

#[derive(Shape)]
struct H {
  #[name = "hi.txt"]
  pub hi: File,
}

#[derive(Shape)]
struct Test {
  #[name = "hi.txt"]
  pub hi: File,
  #[pattern = "*.txt"]
  pub my_dir: Dir,
  pub hi_dir: H,
}

#[test]
fn main_test() -> error::Result<()> {
  let target = Dir::temp_dir_rand()?;
  let shape: Shape<Test> = Shape::new();
  let some = shape.create_at(&target.path)?;

  assert_eq!(some.hi.path, target.path.join("hi.txt"));
  assert_eq!(some.my_dir.path, target.path.join("my_dir"));
  // assert_eq!(some.hi_dir.path, target.path.join("hi_dir"));
  assert_eq!(some.hi_dir.hi.path, target.path.join("hi_dir/hi.txt"));

  assert!(some.hi.exists());
  assert!(some.my_dir.exists());
  assert!(some.hi_dir.hi.exists());
  // assert!(some.hi_dir.exists());

  let res = shape.validate(&target.path);
  if let Err(errors) = res {
    println!("{:?}", errors);
    panic!("validate returned errors");
  }

  okay_to_err(target.delete());
  Ok(())
}
