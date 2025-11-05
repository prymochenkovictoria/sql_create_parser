use anyhow::Result; 
use std::{env, fs, process};

use sql_create_parser::parse_query;

fn print_help() {
    println!("SQL CREATE TABLE Parser CLI");
    println!("Usage:");
    println!("  cargo run -- <command> <file_name>");
    println!("Commands:");
    println!("  parse <file_name>      Parses the specified schema file.");
    println!("  help                   Displays this help message.");
    println!("  credits                Shows credits and author information.");
}

fn print_credits() {
    println!("SQL CREATE TABLE Parser");
    println!("Developed by Victoria Prymochenko");
    println!("\nThis tool parses SQL CREATE TABLE queries and outputs their abstract syntax tree (AST).");
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: No command provided.");
        print_help();
        process::exit(1);
    }

    match args[1].as_str() {
        "parse" => {
            if args.len() < 3 {
                eprintln!("Error: No file name provided for 'parse' command.");
                process::exit(1);
            }
            let file_name = &args[2];

            let content = match fs::read_to_string(file_name) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Failed to read file '{}': {}", file_name, e);
                    process::exit(1);
                }
            };

            match parse_query(&content) {
                Ok(schema) => {
                    println!("Parsed Query:\n{:#?}", schema);
                }
                Err(e) => {
                    eprintln!("Error parsing schema: {}", e);
                    process::exit(1);
                }
            }
        }
        "help" => {
            print_help();
        }
        "credits" => {
            print_credits();
        }
        _ => {
            eprintln!("Error: Unrecognized command '{}'", args[1]);
            print_help();
            process::exit(1);
        }
    }

    Ok(())
}