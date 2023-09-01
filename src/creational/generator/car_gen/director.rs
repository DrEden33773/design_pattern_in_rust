use super::{
  builders::Builder,
  cars::CarType,
  components::{Engine, GPSNavigator, Transmission},
};

pub struct Director;

impl Director {
  pub fn construct_sports_car(builder: &mut impl Builder) {
    builder
      .set_car_type(CarType::SportsCar)
      .set_seats(2)
      .set_engine(Engine::new(3.0, 0.0))
      .set_transmission(Transmission::SemiAutomatic)
      .set_gps_navigator(GPSNavigator::default());
  }

  pub fn construct_city_car(builder: &mut impl Builder) {
    builder
      .set_car_type(CarType::CityCar)
      .set_seats(2)
      .set_engine(Engine::new(1.2, 0.0))
      .set_transmission(Transmission::Automatic)
      .set_gps_navigator(GPSNavigator::default());
  }

  pub fn construct_suv(builder: &mut impl Builder) {
    builder
      .set_car_type(CarType::Suv)
      .set_seats(4)
      .set_engine(Engine::new(2.5, 0.0))
      .set_transmission(Transmission::Manual)
      .set_gps_navigator(GPSNavigator::default());
  }
}
