use std::fmt::Debug;
use std::ops::AddAssign;
use std::pin::Pin;
use std::rc::Rc;

trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>);
}

trait SayHi: Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self);
    }
}

impl<T> SayHi for Box<T> where T: Debug {}

impl<T> MutMeSomehow for Box<T>
where
    T: AddAssign + Copy,
{
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let value = self.get_mut();
        **value += **value;
    }
}

impl<T> SayHi for Rc<T> where T: Debug {}

impl<T> MutMeSomehow for Rc<T>
where
    T: AddAssign + Copy,
{
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let value = self.get_mut();
        let mut v = *value.clone();
        v += v;
    }
}

impl<T> SayHi for Vec<T> where T: Debug {}

impl<T> MutMeSomehow for Vec<T>
where
    T: Unpin,
{
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let vector = self.get_mut();
        vector.reverse();
    }
}

impl SayHi for String {}

impl MutMeSomehow for String {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let value = self.get_mut();
        *value = format!("{}{}", "prefix_", *value);
    }
}

impl SayHi for &[u8] {}

impl MutMeSomehow for &[u8] {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let value = self.get_mut();
        *value = "other bytes".as_bytes();
    }
}

impl SayHi for i32 {}

impl MutMeSomehow for i32 {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let value = self.get_mut();
        *value = *value << 1;
    }
}

fn main() {
    Pin::new(&Box::new(5)).say_hi();
    Pin::new(&mut Box::new(5)).mut_me_somehow();

    Pin::new(&Rc::new(666)).say_hi();
    Pin::new(&mut Rc::new(666)).mut_me_somehow();

    Pin::new(&vec![1, 2, 3]).say_hi();
    Pin::new(&mut vec![1, 2, 3]).mut_me_somehow();

    Pin::new(&String::from("str")).say_hi();
    Pin::new(&mut String::from("str")).mut_me_somehow();

    Pin::new(&"bytes".as_bytes()).say_hi();
    Pin::new(&mut "bytes".as_bytes()).mut_me_somehow();

    Pin::new(&i32::from(10)).say_hi();
    Pin::new(&mut i32::from(10)).mut_me_somehow();
}
