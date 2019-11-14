use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;

struct GlobalStack<T> {
    data: Vec<Rc<RefCell<T>>>,
}

impl<T> GlobalStack<T> {
    pub fn new() -> Self {
        GlobalStack { data: vec![] }
    }

    pub fn push(self: &mut Self, value: Rc<RefCell<T>>) {
        self.data.push(value);
    }
    // pub fn pop(self: &mut Self) -> Option<RefCell<T>> {
    //     if self.data.len() > 0 {
    //         return Some(self.data.pop().unwrap());
    //     } else {
    //         return None;
    //     }
    // }
    pub fn get(self: &mut Self, index: i32) -> RefCell<T> {
        // self.data.get(index as usize).unwrap()
    }
}

fn a_fn_that_mutably_borrows(b: &mut i32) {
    *b += 1;
}

fn main() {
    let mut stack: GlobalStack<i32> = GlobalStack::new();
    let value = Rc::new(RefCell::new(11));

    stack.push(2);
    stack.push(9);
    stack.push(0);
    stack.push(1);

    println!("{:?}", stack.data);

    // let value: RefCell<i32> = stack.get(3);

    println!("{:?}", stack.data);

    // let stack = vec![RefCell::new(11)];

    // let v = stack.get(0).unwrap();

    // a_fn_that_mutably_borrows(&mut v.borrow_mut());

    // println!("{:?}", stack);
}
