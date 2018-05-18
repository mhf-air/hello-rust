#![allow(dead_code)]

extern crate glium;
extern crate libc;

use glium::glutin;
use libc::c_int;

extern "C" {
  fn sum(a: c_int, b: c_int) -> c_int;
}

fn main() {
  gl();
}

fn gl() {
  let mut events_loop = glutin::EventsLoop::new();
  let window = glutin::WindowBuilder::new();
  let context = glutin::ContextBuilder::new();
  let display = glium::Display::new(window, context, &events_loop).unwrap();

  let mut closed = false;
  while !closed {
    events_loop.poll_events(|ev| match ev {
      glutin::Event::WindowEvent { event, .. } => match event {
        glutin::WindowEvent::Closed => closed = true,
        _ => (),
      },
      _ => (),
    });
  }
}

// c ffi
fn run_sum() {
  let a = 1;
  let b = 2;
  let c = unsafe { sum(a, b) };
  println!("{}", c);
}
