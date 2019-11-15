extern crate rand;

use rand::{thread_rng, Rng};

use std::io::{Error, ErrorKind};
use std::ops::Deref;

struct EmailString<'a> {
    value: &'a str,
}

impl<'a> EmailString<'a> {
    pub fn new(value: &'a str) -> Result<Self, Error> {
        if !(value.contains("@") && value.contains(".")) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "String is not a valid email string",
            ));
        }

        Ok(EmailString { value })
    }
}

impl<'a> Deref for EmailString<'a> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

struct Random<'a, T> {
    data: Vec<&'a T>,
}

impl<'a, T> Deref for Random<'a, T> {
    type Target = &'a T;

    fn deref(&self) -> &Self::Target {
        let random: u32 = thread_rng().gen_range(0, 3);

        &self.data.get(random as usize).unwrap()
    }
}

impl<'a, T> Random<'a, T> {
    pub fn new(value1: &'a T, value2: &'a T, value3: &'a T) -> Self {
        Random {
            data: vec![value1, value2, value3],
        }
    }
}

fn main() {
    let valid_email = EmailString::new("Vasya-Pupkin@ya.ru").unwrap();
    let invalid_email = EmailString::new("Vasya-Pupkin@yaru");

    println!("valid email: {}", *valid_email);

    let value1 = "LOL";
    let value2 = "KEK";
    let value3 = "CHEBUREK";

    let random = Random::new(&value1, &value2, &value3);

    println!("Random: {}", *random);
}
