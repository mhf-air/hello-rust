use std::fmt;

pub enum List {
  Cons(i32, Box<List>),
  Nil,
}

impl List {
  pub fn println(&self) {
    println!("{}", self);
  }

  fn string(&self, is_start: bool) -> String {
    let mut content = String::new();
    match self {
      List::Cons(x, cdr) => {
        if !is_start {
          content.push_str(", ");
        }
        content.push_str(&x.to_string());
        content.push_str(cdr.string(false).as_ref());
      }
      List::Nil => {}
    }
    content
  }
}

impl fmt::Display for List {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let result = self.string(true);
    write!(f, "({})", result)
  }
}
