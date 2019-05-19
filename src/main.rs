mod tokens;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() {

    let path = Path::new("program.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => println!("{} contains:\n{}", display, s),
    }

    lexer(s)
}

fn lexer(program: String) {

    let mut tree: Vec<tokens::Tokens> = parse_tokens(program);

    run_program(tree);
}

fn parse_tokens(program: String) -> Vec<tokens::Tokens>{
    let mut parsed = program.split_whitespace();
    let mut vec: Vec<tokens::Tokens> = Vec::new();
    
    for token in parsed {
        match token {
            //_ => vec.push(tokens::Tokens::Null {value : "None".to_string()}), 
            "print" => vec.push(tokens::Tokens::Id {value : token.to_string()}), 
            "<=" | ">=" | "<" | ">" | "=" | "==" | "+" | "-" | "/" | "*" | "%" | "^" => vec.push(tokens::Tokens::Op {value : token.to_string()}),
            //std::i64::MIN...std::i64::MAX => vec.push(tokens::Tokens::Num {value : token.from_str_radix(radix: i64)}),
            _ => print!("stop trying"),
        }
    }
    
    println!("{:?}", vec);
    vec
}

fn run_program(tree: Vec<tokens::Tokens>) {
    
    for i in 0..tree.len() {
        match &tree[i] {
            tokens::Tokens::Id{value} => 
                println!("Printing next {:?}", &tree[i+1])
                /*match value {
                    "print" => println!("{:?}", token),
                    _ => continue,
                }*/,
            _ => continue,

        }
    }
}