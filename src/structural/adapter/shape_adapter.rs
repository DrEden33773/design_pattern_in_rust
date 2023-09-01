pub mod adapters;
pub mod round;
pub mod square;

#[cfg(test)]
mod demo {
  use crate::structural::adapter::shape_adapter::{
    adapters::{Adapter, SquarePegAdapter},
    square::SquarePeg,
  };

  use super::round::{RoundHole, RoundPeg};

  #[test]
  fn run() {
    let hole = RoundHole::new(5.);
    let round_peg = RoundPeg::new(5.);
    println!(
      "`{hole:?}` {} `{round_peg:?}`",
      if hole.fits(&round_peg) {
        "fits"
      } else {
        "doesn't fit"
      }
    );

    for square_peg in &[SquarePeg::new(2.), SquarePeg::new(20.)] {
      println!(
        "`{hole:?}` {} `{square_peg:?}`",
        if hole.fits(&SquarePegAdapter::convert(square_peg)) {
          "fits"
        } else {
          "doesn't fit"
        }
      );
    }
  }
}
