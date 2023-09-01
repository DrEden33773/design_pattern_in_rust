use super::components::{Engine, GPSNavigator, Transmission, TripComputer};

#[derive(Default)]
pub enum CarType {
  #[default]
  CityCar,
  SportsCar,
  SUV,
}

pub struct Car {
  car_type: CarType,
  seats: usize,
  engine: Engine,
  transmission: Transmission,
  trip_computer: TripComputer,
  gps_navigator: GPSNavigator,
  fuel: f64,
}

impl Car {
  pub fn new(
    car_type: CarType,
    seats: usize,
    engine: Engine,
    transmission: Transmission,
    trip_computer: TripComputer,
    gps_navigator: GPSNavigator,
  ) -> Self {
    Self {
      car_type,
      seats,
      engine,
      transmission,
      trip_computer,
      gps_navigator,
      fuel: 0.0,
    }
  }
}

pub struct Manual {
  car_type: CarType,
  seats: usize,
  engine: Engine,
  transmission: Transmission,
  trip_computer: TripComputer,
  gps_navigator: GPSNavigator,
}
