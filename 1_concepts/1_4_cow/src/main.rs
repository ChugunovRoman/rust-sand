use std::borrow::Cow;
use std::env;

struct Config<'a> {
    path: Cow<'a, str>,
}

fn main() {
    let mut config = Config {
        path: Cow::Borrowed("/etc/app/app.conf"),
    };
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => (config.path = Cow::Borrowed(args.get(2).unwrap())),
        _ => match env::var("APP_CONF") {
            Ok(value) => {
                if !value.is_empty() {
                    config.path = Cow::Owned(value);
                }
            }
            Err(err) => eprintln!("Cannot get the APP_CONF env, error: {}", err),
        },
    };

    println!("App config path: {}", config.path);
}
