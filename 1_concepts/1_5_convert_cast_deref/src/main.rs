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

fn main() {
    let valid_email = EmailString::new("Vasya-Pupkin@ya.ru").unwrap();
    let invalid_email = EmailString::new("Vasya-Pupkin@yaru");

    println!("valid email: {}", *valid_email);
}
