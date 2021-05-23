use std::env;
use std::io::Write;
use std::fs;
use std::process::Command;

const PYTHON_BUILD: &[u8] = b"#!/bin/sh\npyinataller --onefile main.py\n";

const RUST_BUILD: &[u8] = b"#!/bin/sh\ncargo build";

fn new_project(language: &String, name: &String) {
    fs::create_dir(name);
    env::set_current_dir(name);
    if language == &String::from("python") {
        Command::new("python3")
            .args(&["-m", "venv", "venv"])
            .spawn();
        let mut build_sh = fs::File::create("build.sh").unwrap();
        build_sh.write(PYTHON_BUILD);
    } else if language == &String::from("rust") {
        Command::new("cargo")
            .args(&["init"])
            .spawn();

        let mut build_sh = fs::File::create("build.sh").unwrap();
        build_sh.write(RUST_BUILD);
    } else {
        println!("{} is not supported language", language);
    }
}

fn init_project(language: &String) {
    if language == String::from("python") {
        let mut build_sh = fs::File::create("build.sh").unwrap();
        build_sh.write(PYTHON_BUILD);
    } else if language == String::from("rust") {
        let mut build_sh = fs::File::create("build.sh").unwrap();
        build_sh.write(RUST_BUILD);
    } else {
        println!("{} is not a supported language", language);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == String::from("new") {
        println!("Creating new project...");
        new_project(&args[2], &args[3]);
    } else if args[1] == String::from("init") {
        init_project(&args[1]);
    } else {
        println!("{:?} Unknown argument", args[1]);
    }
}
