use super::gui::{Button, CheckBox, GuiFactory};

pub fn render(factory: impl GuiFactory) {
  factory.create_button().press();
  factory.create_checkbox().switch();
}

#[cfg(test)]
mod demo {
  use crate::creational::abstract_factory::gui_elements_lib::{
    unix_gui::UnixGuiFactory, windows_gui::WindowsGuiFactory,
  };

  use super::*;

  #[test]
  fn run() {
    if cfg!(windows) {
      render(WindowsGuiFactory)
    } else if cfg!(unix) {
      render(UnixGuiFactory)
    } else {
      panic!("Unsupported OS")
    }
  }
}
