pub struct Engine {
  volume: f64,
  mileage: f64,
  started: bool,
}

pub struct GPSNavigator {
  route: String,
}

#[derive(Default)]
pub enum Transmission {
  #[default]
  SingleSpeed,
  Manual,
  Automatic,
  SemiAutomatic,
}

pub struct TripComputer {}

impl Engine {
  pub fn new(volume: f64, mileage: f64) -> Self {
    Self {
      volume,
      mileage,
      started: false,
    }
  }

  pub fn on(&mut self) {
    self.started = true;
  }

  pub fn off(&mut self) {
    self.started = false;
  }

  pub fn go(&mut self, mileage: f64) {
    if self.started {
      self.mileage += mileage;
    } else {
      println!("Cannot go(), you must start engine first!");
    }
  }

  pub fn started(&self) -> bool {
    self.started
  }

  pub fn volume(&self) -> f64 {
    self.volume
  }

  pub fn mileage(&self) -> f64 {
    self.mileage
  }
}

impl Default for GPSNavigator {
  fn default() -> Self {
    Self {
      route: "[221b, Baker Street, London] -> [Scotland Yard, 8-10 Broadway, London]".into(),
    }
  }
}

impl GPSNavigator {
  pub fn new(route: String) -> Self {
    Self { route }
  }

  pub fn route(&self) -> &str {
    self.route.as_ref()
  }
}
