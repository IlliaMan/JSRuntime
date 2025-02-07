use std::{env, fs, io, path::Path};

fn main() -> io::Result<()> {
    let path = env::args().nth(1).expect("<path> is not provided");
    let path = Path::new(&path);
    if path.extension().and_then(|ext| ext.to_str()) != Some("js") {
        return Err(io::Error::new(io::ErrorKind::Other,"only .js files are accepted"));
    }

    let program = fs::read_to_string(path)?;

    println!("program read:\n{}", program);

    Ok(())
}
