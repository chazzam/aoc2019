
pub fn _print_vec(disp: Vec<i64>, width: usize) {
  _print_vech(disp, width, 0);
}

pub fn _print_vech(disp: Vec<i64>, width: usize, height: usize) {
  println!("[");
  for i in disp.iter().enumerate() {
    print!(" {: >4},", i.1);
    if i.0 != 0 && ((i.0 + 1) % width) == 0 && i.0 + 1 != disp.len() {
      println!("");
    }
    if height > 0 && (i.0 + 1) % (height * width) == 0 && i.0 + 1 != disp.len() {
      println!("");
    }
  }
  println!("\n]");
  return;
}

pub fn _print_vect<T>(disp: Vec<T>, width: usize, height: usize)
    where T: std::fmt::Display {
  println!("[");
  for i in disp.iter().enumerate() {
    print!("{: >4},", i.1);
    if i.0 != 0 && ((i.0 + 1) % width) == 0 && i.0 + 1 != disp.len() {
      println!("");
    }
    if height > 0 && (i.0 + 1) % (height * width) == 0 && i.0 + 1 != disp.len() {
      println!("");
    }
  }
  println!("\n]");
  return;
}
