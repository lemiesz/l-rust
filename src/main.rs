mod scanner;
use std::{env, fs::File, io::Read, panic, path::Path, process::exit};

use scanner::Scanner;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script-name]");
        exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        // runPrompt();
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
    let scanner = Scanner::new(&file_content);
    scanner.scan_tokens();
}
