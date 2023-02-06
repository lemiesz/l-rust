use std::{env, io::Write, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: generate_ast <output directory>");
        exit(64);
    }
    let output_dir = &args[1];
    println!("output_dir: {}", output_dir);
    define_ast(
        output_dir,
        "Expr",
        vec![
            "Binary   : Expr left, Token operator, Expr right",
            "Grouping : Expr expression",
            "Literal  : Object value",
            "Unary    : Token operator, Expr right",
        ],
    );
}

fn define_ast(output_dir: &str, arg: &str, vec: Vec<&str>) {
    let path = format!("{}/{}.rs", output_dir, arg.to_lowercase());
    println!("writing file to path: {}", path);
    // resolve path relative to current directory
    // open a file buffer and write hello world to the file defined on the above path
    let mut file = std::fs::File::create(path).unwrap();
    writeln!(file, "use super::token::Token;").unwrap();
    writeln!(file, "pub struct {} {{", arg).unwrap();
    for field in vec {
        let fields: Vec<&str> = field.split(":").collect();
        let name = fields[0].trim();
        let type_ = fields[1].trim();
        writeln!(file, "    pub {}: {},", name.to_lowercase(), type_).unwrap();
    }
    writeln!(file, "}}").unwrap();
    writeln!(file, "impl {} {{", arg).unwrap();
    writeln!(file, "}}").unwrap();
}
