#[derive(Debug)]
pub struct SquarePeg {
  width: f64,
}

impl SquarePeg {
  pub fn new(width: f64) -> Self {
    Self { width }
  }

  pub fn width(&self) -> f64 {
    self.width
  }

  pub fn square(&self) -> f64 {
    self.width.powi(2)
  }
}
