mod scanner;

use std::{env::{self, Args}, fs, io, path::Path};
use scanner::Scanner;

fn get_js_content(mut args: Args) -> io::Result<String> {
    let path: String = args.nth(1).expect("<path> is not provided");
    let path = Path::new(&path);
    if path.extension().and_then(|ext| ext.to_str()) != Some("js") {
        return Err(io::Error::new(io::ErrorKind::Other,"only .js files are accepted"));
    }

    Ok(fs::read_to_string(path)?)
}

fn main() -> io::Result<()> {
    let source = get_js_content(env::args())?;

    let mut scanner = Scanner::new(source);
    let tokens = scanner.tokenize();
    
    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
