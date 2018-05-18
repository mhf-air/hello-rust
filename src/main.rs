#![allow(dead_code)]
#[macro_use]

extern crate glium;
extern crate libc;

use glium::{glutin, Surface};
use libc::c_int;

extern "C" {
  fn sum(a: c_int, b: c_int) -> c_int;
}

fn main() {
  gl();
}

#[derive(Copy, Clone)]
struct Vertex {
  position: [f32; 2],
}

fn gl() {
  let mut events_loop = glutin::EventsLoop::new();
  let window = glutin::WindowBuilder::new();
  let context = glutin::ContextBuilder::new();
  let display = glium::Display::new(window, context, &events_loop).unwrap();

  implement_vertex!(Vertex, position);

  let vertext1 = Vertex {
    position: [-0.5, -0.5],
  };
  let vertext2 = Vertex {
    position: [0.0, 0.5],
  };
  let vertext3 = Vertex {
    position: [0.5, -0.25],
  };
  let shape = vec![vertext1, vertext2, vertext3];

  let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
  let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

  let vertex_shader_src = r#"
    #version 450

    in vec2 position;

    uniform float t;

    void main() {
      vec2 pos = position;
      pos.x += t;
      gl_Position = vec4(pos, 0.0, 1.0);
    }
  "#;

  let fragment_shader_src = r#"
    #version 450

    out vec4 color;

    void main() {
      color = vec4(1.0, 0.0, 0.0, 1.0);
    }
  "#;

  let program =
    glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

  let mut t: f32 = -0.5;
  let mut closed = false;
  while !closed {
    t += 0.002;
    if t > 0.5 {
      t = -0.5;
    }

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    target
      .draw(
        &vertex_buffer,
        &indices,
        &program,
        &uniform!{t: t},
        &Default::default(),
      )
      .unwrap();
    target.finish().unwrap();

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
