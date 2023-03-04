use std::io::Write;
use std::{env, fs::File, io::Read, panic, path::Path, process::exit};

use common::parser::{self, Parser};
use common::scanner::Scanner;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script-name]");
        exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_prompt() -> () {
    println!("Welcome to rlox! (Type exit to quit)");

    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();

        while input.ends_with(";\n") {
            // append the next line to the input
            let mut next_line = String::new();
            print!("> ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut next_line).unwrap();
            input.push_str(&next_line);
        }

        if input == "exit\n" {
            break;
        }

        run(input.clone());
    }
}

fn run_file(path: &String) {
    let file_content = match File::open(Path::new(path)) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            content
        }
        Err(_) => panic!("Error opening file {}", path),
    };
    run(file_content)
}

fn run(file_content: String) {
    let mut scanner = Scanner::new(file_content);
    scanner.scan_tokens();
    scanner.debug_print();
    let parser = Parser::new(&scanner.tokens);
    match parser.parse() {
        Ok(expr) => {
            println!("Parsed successfully");
            println!("{:?}", &expr.to_string());
            println!("{:?}", expr);
        }
        Err(e) => {
            println!("Error parsing: {}", e);
        }
    }

    println!("Done")
}
