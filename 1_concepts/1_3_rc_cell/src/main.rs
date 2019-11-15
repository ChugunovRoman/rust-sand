use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct GlobalStack<T> {
    data: Rc<RefCell<Vec<T>>>,
}

impl<T> GlobalStack<T> {
    pub fn new() -> Self {
        GlobalStack {
            data: Rc::new(RefCell::new(vec![])),
        }
    }

    pub fn push(self: &Self, value: T) {
        let mut vec = self.data.borrow_mut();
        vec.push(value);
    }
}

fn main() {
    let stack: GlobalStack<i32> = GlobalStack::new();

    stack.push(11);
    stack.push(1);
    stack.push(41);

    let stack2 = stack.clone();

    stack.push(666);

    println!("{:?}", stack.data);
    println!("{:?}", stack2.data);
}
