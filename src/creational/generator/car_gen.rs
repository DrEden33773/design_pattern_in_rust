pub mod builders;
pub mod cars;
pub mod components;
pub mod director;

#[cfg(test)]
mod demo {
  use crate::creational::generator::car_gen::builders::CarManualBuilder;

  use super::{
    builders::{Builder, CarBuilder},
    director::Director,
  };

  #[test]
  fn run() {
    let mut car_builder = CarBuilder::default();
    Director::construct_sports_car(&mut car_builder);
    let car = car_builder.build();
    println!("Car built: {:?}\n", car.car_type());

    let mut manual_builder = CarManualBuilder::default();
    Director::construct_sports_car(&mut manual_builder);
    let manual = manual_builder.build();
    println!("Car manual built:\n{}", manual);
  }
}
