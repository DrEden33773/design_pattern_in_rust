use super::gui::{Button, CheckBox, DynamicGuiFactory, GuiFactory};

pub struct WindowsButton;
impl Button for WindowsButton {
  fn press(&self) {
    println!("Pressed: Windows.Button")
  }
}

pub struct WindowsCheckBox;
impl CheckBox for WindowsCheckBox {
  fn switch(&self) {
    println!("Switched: Windows.CheckBox")
  }
}

pub struct WindowsGuiFactory;
impl GuiFactory for WindowsGuiFactory {
  type B = WindowsButton;
  type C = WindowsCheckBox;
  fn create_button(&self) -> Self::B {
    WindowsButton
  }
  fn create_checkbox(&self) -> Self::C {
    WindowsCheckBox
  }
}

impl DynamicGuiFactory for WindowsGuiFactory {
  fn create_button(&self) -> Box<dyn Button> {
    Box::new(WindowsButton)
  }
  fn create_checkbox(&self) -> Box<dyn CheckBox> {
    Box::new(WindowsCheckBox)
  }
}
