use std::fs::File;
use std::io;
mod tokenizer;

fn main() -> io::Result<()> {
    let f = File::open("/home/mahmoud/compiler/src/main.dod")?;
    
    let tokenizer = tokenizer::Tokenizer::new(f);

    for token in tokenizer {
        println!("{:?}", token.kind);
    }
    
    Ok(())
}
