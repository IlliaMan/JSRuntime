mod scanner;

use std::{env, fs, io, path::Path};
use scanner::Scanner;

fn main() -> io::Result<()> {
    let path = env::args().nth(1).expect("<path> is not provided");
    let path = Path::new(&path);
    if path.extension().and_then(|ext| ext.to_str()) != Some("js") {
        return Err(io::Error::new(io::ErrorKind::Other,"only .js files are accepted"));
    }

    let source = fs::read_to_string(path)?;
    let mut scanner = Scanner::new(source);

    let tokens = scanner.tokenize();
    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
