extern crate libc;
use libc::c_int;

extern "C" {
  fn sum(a: c_int, b: c_int) -> c_int;
}

fn main() {
  let a = 1;
  let b = 2;
  let c = unsafe { sum(a, b) };
  println!("{}", c);
}
