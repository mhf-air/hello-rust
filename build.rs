extern crate cc;

fn main() {
  cc::Build::new().file("src/sum.c").compile("sum");
}
