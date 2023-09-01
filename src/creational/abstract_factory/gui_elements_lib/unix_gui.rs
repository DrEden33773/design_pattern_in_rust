use super::gui::{Button, CheckBox, DynamicGuiFactory, GuiFactory};

pub struct UnixButton;
impl Button for UnixButton {
  fn press(&self) {
    println!("Pressed: Unix.Button")
  }
}

pub struct UnixCheckBox;
impl CheckBox for UnixCheckBox {
  fn switch(&self) {
    println!("Switched: Unix.CheckBox")
  }
}

pub struct UnixGuiFactory;
impl GuiFactory for UnixGuiFactory {
  type B = UnixButton;
  type C = UnixCheckBox;
  fn create_button(&self) -> Self::B {
    UnixButton
  }
  fn create_checkbox(&self) -> Self::C {
    UnixCheckBox
  }
}

impl DynamicGuiFactory for UnixGuiFactory {
  fn create_button(&self) -> Box<dyn Button> {
    Box::new(UnixButton)
  }
  fn create_checkbox(&self) -> Box<dyn CheckBox> {
    Box::new(UnixCheckBox)
  }
}
