#![allow(dead_code)]

fn main() {
  // run_list();
  th::run_shared();
}

// ================================================================================
#[macro_use]
extern crate glium;
mod gl;

mod c;
mod th;

mod list;
fn run_list() {
  use list::List::{Cons, Nil};
  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
  list.println();
}
