use std::env::{self};

#[derive(PartialEq)]
enum Command {
    None,
    Init,
    Commit,
    Log,
    Checkout
}

struct Option {
    option: String,
    value: String
}

impl Option {
    pub fn new(option: String, value: String) -> Option {
        Option {
            option: option,
            value: value
        }
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    // Remove the file path from the list of arguments
    args.remove(0);
    let mut options: Vec<Option> = Vec::new();
    let argument = &get_argument(0, &args).expect("Argument Parsing").to_lowercase()[..];
    let command = match argument {
        "init" => Command::Init,
        "commit" => Command::Commit,
        "log" => Command::Log,
        "checkout" => Command::Checkout,
        _ => {
            eprintln!("You have not entered a command in!");
            Command::None
        }
    };
    for i in 0..args.len() {
        let argument = &get_argument(i, &args).expect("Argument Parsing").to_lowercase()[..];
        if argument.chars().next() == Some('-') {
            options.push(option_parsing(argument, i, &args));
        }
    }
    if command == Command::Init {
        // Initialize a git repository
        println!("Initializing github repo!");
    }
    else if command == Command::Commit {
        // Commit changes to git repository
        println!("Commiting changes to repo!");
    }
    else if command == Command::Log {
        // Output log of git repository
        println!("Outputting commit log!");
    }
    else if command == Command::Checkout {
        // Switch repository branch
        println!("Switching repository branch!");
    }
    else {
        eprintln!("You have not entered a command in!");
    }
}

fn get_argument(index: usize, args: &Vec<String>) -> Result<String, String> {
    let initial_get = args.get(index);
    if let Some(initial_get) = initial_get {
        Ok(initial_get.to_owned())
    }
    else {
        Err(format!("Couldn't get argument at index: {index}"))
    }
}

fn option_parsing(argument: &str, i: usize, args: &Vec<String>) -> Option {
    let key = argument;
    let mut values: Vec<String> = Vec::new();
    if i+1 == args.len() {
        panic!("Missing value for option: {key}");
    }
    for j in i+1..args.len() {
        let value = get_argument(j, args).ok();
        if let Some(x) = value {
            if x.chars().next() == Some('-') {
                break;
            }
            values.push(x);
        }
    }
    if values.len() == 0 {
        panic!("Missing value for option: {key}");
    }
    let value = values.join(" ");

    Option::new(key.to_string(), value)
}