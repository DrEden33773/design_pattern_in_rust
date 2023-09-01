use super::{gui::DynamicGuiFactory, unix_gui::UnixGuiFactory, windows_gui::WindowsGuiFactory};

pub fn render(factory: &dyn DynamicGuiFactory) {
  factory.create_button().press();
  factory.create_checkbox().switch();
}

pub fn get_factory() -> Box<dyn DynamicGuiFactory> {
  if cfg!(windows) {
    Box::new(WindowsGuiFactory)
  } else if cfg!(unix) {
    Box::new(UnixGuiFactory)
  } else {
    panic!("Unsupported OS")
  }
}

#[cfg(test)]
mod demo {
  use super::*;

  #[test]
  fn run() {
    let factory = get_factory();
    render(factory.as_ref());
  }
}
