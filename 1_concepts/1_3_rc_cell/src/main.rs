use std::cell::RefCell;
use std::rc::Rc;

struct GlobalStack<T> {
    data: Vec<Rc<RefCell<T>>>,
}

impl<T> GlobalStack<T> {
    pub fn new() -> Self {
        GlobalStack { data: vec![] }
    }

    pub fn push(self: &mut Self, value: T) {
        self.data.push(Rc::new(RefCell::new(value)));
    }
    pub fn pop(self: &mut Self) -> Option<Rc<RefCell<T>>> {
        if self.data.len() > 0 {
            return Some(self.data.get(self.data.len() - 1).unwrap().clone());
        } else {
            return None;
        }
    }
}

fn main() {
    let mut stack: GlobalStack<i32> = GlobalStack::new();

    stack.push(11);
    stack.push(1);
    stack.push(41);

    println!("{:?}", stack.data);

    let value: Rc<RefCell<i32>> = stack.pop().unwrap();
    *value.try_borrow_mut().unwrap() -= 10;

    println!("{:?}", stack.data);

    stack.push(666);

    let value2: Rc<RefCell<i32>> = stack.pop().unwrap();
    *value2.try_borrow_mut().unwrap() -= 10;

    println!("{:?}", stack.data);
}
