mod common;
mod tokenizer;
mod parser;
mod runtime;

use parser::Parser;
use runtime::Runtime;
use tokenizer::Tokenizer;
use std::{env::{self, Args}, fs, io, path::Path};

fn get_js_content(mut args: Args) -> io::Result<String> {
    let path: String = args.nth(1).expect("<path> is not provided");
    let path = Path::new(&path);
    if path.extension().and_then(|ext| ext.to_str()) != Some("js") {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "only .js files are accepted",
        ));
    }

    Ok(fs::read_to_string(path)?)
}

fn main() -> io::Result<()> {
    let source = get_js_content(env::args())?;

    let mut tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.tokenize();

    for token in tokens.iter() {
        println!("{:?}", token);
    }

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => {
            println!("AST: {:#?}", ast);
            let mut runtime = Runtime::new();
            let _ = runtime.interpret(ast);
        }
        Err(e) => eprintln!("Parse error: {}", e),
    };

    Ok(())
}
