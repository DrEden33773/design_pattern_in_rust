#[derive(Debug)]
pub struct RoundHole {
  radius: f64,
}

impl RoundHole {
  pub fn new(radius: f64) -> Self {
    Self { radius }
  }

  pub fn radius(&self) -> f64 {
    self.radius
  }

  pub fn fits(&self, peg: &RoundPeg) -> bool {
    self.radius >= peg.radius()
  }
}

#[derive(Debug)]
pub struct RoundPeg {
  radius: f64,
}

impl RoundPeg {
  pub fn new(radius: f64) -> Self {
    Self { radius }
  }

  pub fn radius(&self) -> f64 {
    self.radius
  }
}
