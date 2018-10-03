extern crate regex;

use std::process::{Command, exit};
use std::env;
use regex::Regex;

fn main() {
    let output = String::from_utf8(
        Command::new("git")
            .arg("var")
            .arg("GIT_AUTHOR_IDENT")
            .output()
            .expect("Failed to execute git!")
            .stdout
    ).unwrap();
    let git_author_ident = output.as_str();    
    let regex = Regex::new(r"^(.*) <(.*)>").unwrap();
    let captures = regex.captures(&git_author_ident).unwrap();
    let name = captures.get(1).unwrap().as_str();
    let email = captures.get(2).unwrap().as_str();

    check_author(&name, &email);
}

fn check_author(name: &str, email: &str) {
    let valid_name = env::var("NAME").unwrap();
    let valid_email = env::var("EMAIL").unwrap();
    let is_name_matched = name == valid_name;
    let is_email_matched = email == valid_email;
    
    match (is_name_matched, is_email_matched) {
        (false, true) => {
            println!("You tried to commit with this name({})", name);
            println!("Please change your name correctly using the following command.\n");
            println!("    $ git config user.name {}", valid_name);
            exit(1);
        },
        (true, false) => {
            println!("You tried to commit with this email({})", email);
            println!("Please change your email correctly using the following command.\n");
            println!("    $ git config user.email {}", valid_email);
            exit(1);
        },
        (false, false) => {
            println!("You tried to commit with this name({}) and email({})", name, email);
            println!("Please change your name and email correctly using the following commands.\n");
            println!("    $ git config user.name {}", valid_name);
            println!("    $ git config user.email {}", valid_email);
            exit(1);
        },
        (true, true) => exit(0)
    }
}
