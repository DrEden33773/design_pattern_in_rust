use super::{
  cars::{Car, CarType, Manual},
  components::{Engine, GPSNavigator, Transmission},
};

pub const DEFAULT_FUEL: f64 = 5f64;

pub trait Builder {
  type Output;
  fn set_car_type(&mut self, car_type: CarType) -> &mut Self;
  fn set_seats(&mut self, seats: u16) -> &mut Self;
  fn set_engine(&mut self, engine: Engine) -> &mut Self;
  fn set_transmission(&mut self, transmission: Transmission) -> &mut Self;
  fn set_gps_navigator(&mut self, gps_navigator: GPSNavigator) -> &mut Self;
  fn set_fuel(&mut self, fuel: f64) -> &mut Self;
  fn build(self) -> Self::Output;
}

#[derive(Default)]
pub struct CarBuilder {
  car_type: Option<CarType>,
  engine: Option<Engine>,
  gps_navigator: Option<GPSNavigator>,
  seats: Option<u16>,
  transmission: Option<Transmission>,
  fuel: Option<f64>,
}

impl Builder for CarBuilder {
  type Output = Car;

  fn set_car_type(&mut self, car_type: CarType) -> &mut Self {
    self.car_type = Some(car_type);
    self
  }

  fn set_seats(&mut self, seats: u16) -> &mut Self {
    self.seats = Some(seats);
    self
  }

  fn set_engine(&mut self, engine: Engine) -> &mut Self {
    self.engine = Some(engine);
    self
  }

  fn set_transmission(&mut self, transmission: Transmission) -> &mut Self {
    self.transmission = Some(transmission);
    self
  }

  fn set_gps_navigator(&mut self, gps_navigator: GPSNavigator) -> &mut Self {
    self.gps_navigator = Some(gps_navigator);
    self
  }

  fn set_fuel(&mut self, fuel: f64) -> &mut Self {
    self.fuel = Some(fuel);
    self
  }

  fn build(self) -> Self::Output {
    Car::new(
      self.car_type.expect("Please, set a car type"),
      self.seats.expect("Please, set a number of seats"),
      self.engine.expect("Please, set an engine configuration"),
      self.transmission.expect("Please, set up transmission"),
      self.gps_navigator,
      self.fuel.unwrap_or(DEFAULT_FUEL),
    )
  }
}

#[derive(Default)]
pub struct CarManualBuilder {
  car_type: Option<CarType>,
  engine: Option<Engine>,
  gps_navigator: Option<GPSNavigator>,
  seats: Option<u16>,
  transmission: Option<Transmission>,
}

impl Builder for CarManualBuilder {
  type Output = Manual;

  fn set_car_type(&mut self, car_type: CarType) -> &mut Self {
    self.car_type = Some(car_type);
    self
  }

  fn set_seats(&mut self, seats: u16) -> &mut Self {
    self.seats = Some(seats);
    self
  }

  fn set_engine(&mut self, engine: Engine) -> &mut Self {
    self.engine = Some(engine);
    self
  }

  fn set_transmission(&mut self, transmission: Transmission) -> &mut Self {
    self.transmission = Some(transmission);
    self
  }

  fn set_gps_navigator(&mut self, gps_navigator: GPSNavigator) -> &mut Self {
    self.gps_navigator = Some(gps_navigator);
    self
  }

  fn set_fuel(&mut self, _fuel: f64) -> &mut Self {
    self
  }

  fn build(self) -> Self::Output {
    Manual::new(
      self.car_type.expect("Please, set a car type"),
      self.seats.expect("Please, set a number of seats"),
      self.engine.expect("Please, set an engine configuration"),
      self.transmission.expect("Please, set up transmission"),
      self.gps_navigator,
    )
  }
}
