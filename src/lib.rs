use std::error::Error;
use std::fs;


pub struct Config {
    pub md_path: String,
    pub output_path: String
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skip path of executable

        // Read program arguments
        let md_path = match args.next() {
            Some(path) => path,
            None => return Err("No markdown file specified"),
        };

        let output_path = match args.next() {
            Some(path) => path,
            None => "output.html".to_string()
        };

        // TODO: Error check arguments <md_path>, <output_path>
        // ...

        Ok(Config { md_path, output_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // let contents: String = match fs::read_to_string(&config.md_path) {
    //     Ok(text) => text,
    //     Err(e) => { return Err(Box::new(e)) }
    // };

    // LEXER
    let tokens: Vec<Vec<Token>> = lexer(&config); // THIS NEEDS TO BE SOME 2D VECTOR OF TOKENS

    // AST
    // WRITE

    // for line in tokens {
    //     println!("<------->");
    //     for word in line {
    //         println!("token: {}", word.value);
    //     }
    // }

    Ok(())
}


pub enum TokenType {
    Prefix, Whole, Text
}

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

fn lexer(config: &Config) -> Vec<Vec<Token>> {
    let contents: String = match fs::read_to_string(&config.md_path) {
        Ok(text) => text,
        // Err(e) => { return Err(Box::new(e)) }
        Err(_e) => { return Vec::new(); }
    };

    let mut output: Vec<Vec<Token>> = Vec::new();
    for line in contents.lines() {
        let mut line_tokens: Vec<Token> = Vec::new();

        let words: Vec<&str> = line.split(" ").collect();
        
        if words.len() == 0 {
            line_tokens.push(Token{ token_type: TokenType::Whole, value: String::from("empty_line") })
        } else if words.len() == 1 {
            if line.chars().all(|c| c == '=') {
                line_tokens.push(Token{ token_type: TokenType::Whole, value: String::from("h1") })
            } else if line.chars().all(|c| c == '-') {
                line_tokens.push(Token{ token_type: TokenType::Whole, value: String::from("h2") })
            } else {
                line_tokens.push(Token{ token_type: TokenType::Text, value: String::from(line) })
            }
        } else {
            let prefix: &str = words[0];
            if prefix.chars().all(|c| c == '#') {
                line_tokens.push(Token{ token_type: TokenType::Prefix, value:String::from("h1") }); // TODO: should not always be h1
                line_tokens.push(Token{ token_type: TokenType::Text, value: String::from(&line[prefix.len()..]) });
            } else {
                line_tokens.push(Token{ token_type: TokenType::Text, value: String::from(line) });
            }
        }


        output.push(line_tokens);
    }
    output
}


fn ast(tokens: Vec<Vec<Token>>) -> () {

}
