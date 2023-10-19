use crate::ast::lexer::Lexer;
use crate::ast::token::{TokenType};
use std::fs::File;
use std::io::Read;
mod ast;

fn main() {
    let file_path = "src/language_test/simple_declaration.txt";
    let mut file = File::open(file_path).expect("Failed to open file");

    // Read the contents of the file into a String
    let mut input_string = String::new();
    file.read_to_string(&mut input_string).expect("Failed to read file");

    let mut lexer = Lexer {
        input: String::from(""),
        position: 0,
        read_position: 0,
        ch: '\0', // Initialize with a default value, e.g., '\0'
    };
    Lexer::new(&mut lexer, input_string);
    let mut tok = lexer.next_token();

    while tok.token_type != TokenType::EOF {
        println!("{:?}", tok);
        if tok.token_type == TokenType::ILLEGAL {
            break;
        }
        tok = lexer.next_token();
    }
}