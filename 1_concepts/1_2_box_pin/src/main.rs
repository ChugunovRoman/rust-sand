use std::fmt::{self, Debug, Display};
use std::pin::Pin;
use std::rc::Rc;

trait MutMeSomehow {
    fn mut_me_somehow(self: &mut Self);
}

trait SayHi {
    fn say_hi(self: &Self) {
        println!("Hi from");
    }
}

impl<T> SayHi for Box<T>
where
    T: Debug,
{
    fn say_hi(self: &Self) {
        println!("Hi from Box: {:?}", self)
    }
}

// impl<i32> MutMeSomehow for Box<i32> {
//     fn mut_me_somehow(self: &mut Self) {
//         let mut value = self.as_mut();
//         value = value + 2;
//     }
// }

// impl<T> SayHi for Rc<T> {
//     fn say_hi(self: Pin<&Self>) {
//         println!("Hi from Rc")
//     }
// }

// impl<T> MutMeSomehow for Rc<T> {
//     fn mut_me_somehow(self: Pin<&mut Self>) {}
// }

// impl<T> SayHi for Vec<T> {
//     fn say_hi(self: Pin<&Self>) {
//         println!("Hi from Vec")
//     }
// }

// impl<T> MutMeSomehow for Vec<T> {
//     fn mut_me_somehow(self: Pin<&mut Self>) {
//         let element = &self;
//     }
// }

// impl SayHi for String {
//     fn say_hi(self: Pin<&Self>) {
//         println!("Hi from {:?}", self);
//     }
// }

// impl MutMeSomehow for String {
//     fn mut_me_somehow(self: Pin<&mut Self>) {
//         *self = format!("{}{}", "prefix_", *self);
//     }
// }

impl SayHi for &[u8] {
    fn say_hi(self: &Self) {
        println!("Hi from {:?}", self);
    }
}

// impl MutMeSomehow for &[u8] {
//     fn mut_me_somehow(self: &mut Self) {
//         *self.reverse();
//     }
// }

impl SayHi for i32 {
    fn say_hi(self: &Self) {
        println!("Hi from {:?}", self);
    }
}

impl MutMeSomehow for i32 {
    fn mut_me_somehow(self: &mut Self) {
        *self = *self << 1;
    }
}

fn main() {
    let b = Box::new(1);
    b.say_hi();

    // let s = "".to_owned();
    // println!("Box: {}", b);

    let mut num: i32 = 5;
    num.say_hi();
    num.mut_me_somehow();
    println!("num: {}", num);
}
