use std::fmt::Display;

use super::components::{Engine, GPSNavigator, Transmission};

#[derive(Copy, Clone, Debug, Default)]
pub enum CarType {
  #[default]
  CityCar,
  SportsCar,
  Suv,
}

pub struct Car {
  car_type: CarType,
  seats: u16,
  engine: Engine,
  transmission: Transmission,
  gps_navigator: Option<GPSNavigator>,
  fuel: f64,
}

impl Car {
  pub fn new(
    car_type: CarType,
    seats: u16,
    engine: Engine,
    transmission: Transmission,
    gps_navigator: Option<GPSNavigator>,
    fuel: f64,
  ) -> Self {
    Self {
      car_type,
      seats,
      engine,
      transmission,
      gps_navigator,
      fuel,
    }
  }

  pub fn car_type(&self) -> &CarType {
    &self.car_type
  }

  pub fn seats(&self) -> u16 {
    self.seats
  }

  pub fn engine(&self) -> &Engine {
    &self.engine
  }

  pub fn transmission(&self) -> &Transmission {
    &self.transmission
  }

  pub fn gps_navigator(&self) -> Option<&GPSNavigator> {
    self.gps_navigator.as_ref()
  }

  pub fn fuel(&self) -> f64 {
    self.fuel
  }

  pub fn set_fuel(&mut self, fuel: f64) {
    self.fuel = fuel;
  }
}

pub struct Manual {
  car_type: CarType,
  seats: u16,
  engine: Engine,
  transmission: Transmission,
  gps_navigator: Option<GPSNavigator>,
}

impl Display for Manual {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "Type of car: {:?}", self.car_type)?;
    writeln!(f, "Count of seats: {}", self.seats)?;
    writeln!(
      f,
      "Engine: volume - {}; mileage - {}",
      self.engine.volume(),
      self.engine.mileage()
    )?;
    writeln!(f, "Transmission: {:?}", self.transmission)?;
    match self.gps_navigator {
      Some(_) => writeln!(f, "GPS Navigator: Functional")?,
      None => writeln!(f, "GPS Navigator: N/A")?,
    };
    Ok(())
  }
}

impl Manual {
  pub fn new(
    car_type: CarType,
    seats: u16,
    engine: Engine,
    transmission: Transmission,
    gps_navigator: Option<GPSNavigator>,
  ) -> Self {
    Self {
      car_type,
      seats,
      engine,
      transmission,
      gps_navigator,
    }
  }
}
