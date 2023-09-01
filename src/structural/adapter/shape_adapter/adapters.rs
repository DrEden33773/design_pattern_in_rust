use std::ops::{Div, Mul};

use super::{round::RoundPeg, square::SquarePeg};

pub trait Adapter<Source, Target> {
  fn convert(source: &Source) -> Target;
}

pub struct SquarePegAdapter;

impl Adapter<SquarePeg, RoundPeg> for SquarePegAdapter {
  fn convert(source: &SquarePeg) -> RoundPeg {
    RoundPeg::new(source.width().div(2.0).mul(2.0_f64.sqrt()))
  }
}
