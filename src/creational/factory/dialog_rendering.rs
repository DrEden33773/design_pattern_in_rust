pub trait Button {
  fn render(&self);
  fn on_click(&self);
}

pub trait Dialog {
  fn create_button(&self) -> Box<dyn Button>;

  fn render(&self) {
    let button = self.create_button();
    button.render()
  }

  fn refresh(&self) {
    println!("Dialog::Refresh")
  }
}

pub struct HtmlButton;

impl Button for HtmlButton {
  fn render(&self) {
    println!("<button> Test Button </button>");
    self.on_click();
  }

  fn on_click(&self) {
    println!("HtmlButton::OnClick -> 'Hello, world!'")
  }
}

pub struct HtmlDialog;

impl Dialog for HtmlDialog {
  fn create_button(&self) -> Box<dyn Button> {
    Box::new(HtmlButton)
  }
}

pub struct WindowsButton;

impl Button for WindowsButton {
  fn render(&self) {
    println!("WindowsButton::Render")
  }

  fn on_click(&self) {
    println!("WindowsButton::OnClick -> 'Hello, world!'")
  }
}

pub struct WindowsDialog;

impl Dialog for WindowsDialog {
  fn create_button(&self) -> Box<dyn Button> {
    Box::new(WindowsButton)
  }
}

pub struct UnixButton;

impl Button for UnixButton {
  fn render(&self) {
    println!("UnixButton::Render")
  }

  fn on_click(&self) {
    println!("UnixButton::OnClick -> 'Hello, world!'")
  }
}

pub struct UnixDialog;

impl Dialog for UnixDialog {
  fn create_button(&self) -> Box<dyn Button> {
    Box::new(UnixButton)
  }
}

pub fn init_dialog() -> Box<dyn Dialog> {
  if cfg!(windows) {
    Box::new(WindowsDialog)
  } else if cfg!(unix) {
    Box::new(UnixDialog)
  } else {
    Box::new(HtmlDialog)
  }
}

#[cfg(test)]
mod demo {
  use super::*;

  #[test]
  fn run() {
    let dialog = init_dialog();
    dialog.render();
    dialog.refresh();
  }
}
