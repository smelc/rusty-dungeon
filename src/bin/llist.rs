use std::fmt::{self, Display};

struct Cell<T> {
    value: T,
    next: Option<Box<Cell<T>>>,
}

impl<T> Cell<T> {
    fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

impl<T: Display> Display for Cell<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)?;
        match &self.next {
            Some(val) => write!(f, "-> {}", val),
            None => Ok(()),
        }
    }
}

fn main() {
    let x: Cell<i32> = Cell::new(0);
    println!("Hello, this is llist: {}", x);
}
