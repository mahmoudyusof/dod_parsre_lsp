use std::fs::File;
use std::io;
mod parser;
mod tokenizer;

fn main() -> io::Result<()> {
    let f = File::open("/home/mahmoud/compiler/src/main.dod")?;

    let tokenizer = tokenizer::Tokenizer::new(f);

    let mut parser = parser::Parser::new(tokenizer);

    parser.parse();

    for stmt in parser.program {
        print_stmt_recursive(stmt, 0);
    }

    Ok(())
}


fn print_stmt_recursive(stmt: parser::Stmt, level: i32) {
    for _ in 0..level {
        print!("--");
    }
    match stmt {
        parser::Stmt::INTEGERLITERAL(n) => {
            println!("{:?}", n);
        }
        parser::Stmt::FLOATLITERAL(n) => {
            println!("{:?}", n);
        }
        parser::Stmt::IDENTIFIER(s) => {
            println!("{:?}", s);
        }
        parser::Stmt::BINARYEXPR(a, b, c) => {
            println!("{:?}", c.kind);
            print_stmt_recursive(*a, level + 1);
            print_stmt_recursive(*b, level + 1);
        }
        parser::Stmt::UNARY(a) => {
            println!("!");
            print_stmt_recursive(*a, level + 1);
        }
        parser::Stmt::PARENTHESIZED(a) => {
            println!("(");
            print_stmt_recursive(*a, level + 1);

            for _ in 0..level {
                print!("--");
            }
            println!(")");
        }
    }
}
