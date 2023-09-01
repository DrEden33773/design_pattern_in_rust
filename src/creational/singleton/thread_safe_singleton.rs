use std::sync::Mutex;

static ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);

pub fn add_to_array(value: u8) {
  ARRAY.lock().unwrap().push(value);
}

#[cfg(test)]
mod demo {
  use std::collections::HashSet;

  use super::*;

  #[test]
  fn run() {
    let mut handles = vec![];
    for i in 0..10 {
      handles.push(std::thread::spawn(move || {
        add_to_array(i);
      }));
    }
    for handle in handles {
      handle.join().unwrap();
    }
    println!("{:?}", ARRAY.lock().unwrap());
    assert_eq!(
      ARRAY
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .collect::<HashSet<_>>(),
      (0..10).collect::<HashSet<_>>()
    )
  }
}
