use std::fmt::Display;

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Circle {
  pub x: i32,
  pub y: i32,
  pub radius: i32,
}

impl Display for Circle {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[allow(dead_code)]
impl Circle {
  pub fn new(x: i32, y: i32, radius: i32) -> Self {
    Self { x, y, radius }
  }
}

impl AsRef<Circle> for Circle {
  fn as_ref(&self) -> &Circle {
    self
  }
}

impl AsMut<Circle> for Circle {
  fn as_mut(&mut self) -> &mut Circle {
    self
  }
}

#[cfg(test)]
mod demo {
  use super::*;

  #[test]
  fn run() {
    let mut circle = Circle::new(10, 15, 10);
    let deep_copied = circle.clone();
    let shallow_copied = circle.as_mut();
    shallow_copied.y = 10;
    println!("Deep copied: {deep_copied}");
    println!("Shallow copied: {shallow_copied}")
  }
}
