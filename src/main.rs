#![allow(dead_code)]
use colored::Colorize;
use std::env;
use std::path::*;
mod compiler;
use compiler::compile;
mod function;
mod tokenizer;
mod parser;
fn main() {
    println!("");
    let file = ensure_valid_root_file(env::args().collect());
    match file {
        Some(path) => compile(path),
        None => {
            error("Compiler failed on step: validate root file.\n");
            return;
        }
    }
}

fn ensure_valid_root_file(mut args: Vec<String>) -> Option<String> {
    args.remove(0);
    if args.len() != 1 {
        error("you must pass one file path to the compiler, it will find the rest.\n");
        return None;
    }
    let file = args[0].clone();
    drop(args);
    if !Path::new(&file).exists() {
        error("the chosen file doesn't exist.");
        return None;
    }

    let file_components: Vec<String> = file.split('.').map(String::from).collect();
    if file_components.len() < 2 {
        error("please pass a file with a file extension.");
        return None;
    }
    if file_components[1] != "c4l" {
        error("please pass a file with the .c4l extension.");
        return None;
    }

    println!("chosen root file: {}", file.bright_green());
    return Some(file);
}

fn error(message: &str) {
    eprintln!("{}", message.to_string().red());
    return;
}
