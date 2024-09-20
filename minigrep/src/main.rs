use std::env;
use std::process;

use minigrep::Config;

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    let mut v = vec![1, 2, 3];
    let third: &i32 = &v[2];

    println!("The third number is {third}");
    v.push(4);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third number is {third}"),
        None => println!("There is no third number."),
    }

    println!("{:?}", v);

    let mut v = vec![100, 32, 57];
    for n_ref in &mut v {
        *n_ref += 50;
        println!("{n_ref}");
    }

    println!("{:?}", v);
}
