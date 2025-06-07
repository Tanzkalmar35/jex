use lexer::lexer::Lexer;
use utils::file_reader::FileReader;

pub(crate) mod lexer;
pub(crate) mod parser;
pub(crate) mod utils;

fn main() {
    let file = "test.jex";
    let file_content = FileReader::read(file)
        .expect("Could not read file. Please verify that the given file exists");
    let content_str = std::str::from_utf8(&file_content).unwrap();

    // Lexing phase
    match Lexer::tokenize(content_str) {
        Ok(tokens) => {
            println!("Lexing successful! Found {} tokens:", tokens.len());
            for (i, token_info) in tokens.iter().enumerate() {
                println!("  {}: {:?} = '{}'", i, token_info.token, token_info.text);
            }

            // Now you can pass `tokens` to your parser

            // Serialize data for language wrappers to handle
        }
        Err(e) => {
            eprintln!("Lexing failed: {}", e);
        }
    }
}
