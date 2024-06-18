use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
//use compiler::lsp::{parse_notification, parse_request, serialize_response, Capabilities, DidOpenNotification, InitializeResult, Request, Response, ServerInfo};
use compiler::parser::{Expr, Parser, Stmt};
use compiler::tokenizer::{TokenKind, Tokenizer};
use compiler::types::diagnostics::{
    Diagnostic, PublishDiagnosticsNotification, PublishDiagnosticsParams,
};
use compiler::types::did_change::{parse_change_notification, DidChangeNotification};
use compiler::types::did_save::{parse_save_notification, DidSaveNotification};
use compiler::types::initialization::DiagnosticOptions;
use compiler::types::{
    did_open::{parse_open_notification, DidOpenNotification},
    initialization::{Capabilities, InitializationResponse, InitializeResult, ServerInfo},
    parse_any, serialize_response, Request,
};
use compiler::types::{Position, Range};

fn main() -> io::Result<()> {
    //let mut f = File::open("/home/mahmoud/compiler/src/main.dod")?;
    //let mut code = String::new();
    //f.read_to_string(&mut code)?;
    //let tokenizer = Tokenizer::tokenize(code.as_str());
    //
    //let mut parser = Parser::new(&tokenizer);
    //
    //parser.parse();
    ////let mut env: HashMap<String, f32> = HashMap::new();
    //
    //for stmt_expr in parser.program {
    //    print_stmt(&stmt_expr, 0);
    //    println!("================");
    //}

    
    let mut f = File::create("/home/mahmoud/compiler/src/log.txt")?;
    loop {
        let msg = read_message();
        match msg {
            Ok(msg) => {
                let request: Request = parse_any(&msg).unwrap();

                f.write_fmt(format_args!(
                    "\nrecieved: {} - {} - {}\n",
                    request.id.unwrap_or(1),
                    request.jsonrpc,
                    request.method
                ))?;
                if request.method == "initialize" {
                    let response = InitializationResponse {
                        jsonrpc: String::from("2.0"),
                        id: request.id.unwrap_or(1),
                        result: InitializeResult {
                            serverInfo: ServerInfo {
                                name: String::from("customLSP"),
                            },
                            capabilities: Capabilities {
                                textDocumentSync: 1,
                                documentHighlightProvider: true,
                                colorProvider: true,
                            },
                        },
                    };
                    let response = serialize_response(response);
                    f.write_fmt(format_args!("\r\n\r\n{}\r\n\r\n", response))?;
                    io::stdout().write_all(response.as_bytes())?;
                    io::stdout().flush()?;
                } else if request.method == "initialized" {
                    f.write(b"\ninitialized\n")?;
                } else if request.method == "textDocument/didOpen" {
                    let notification: DidOpenNotification = parse_open_notification(&msg);
                    //for token in tokens {
                    //    f.write_fmt(format_args!("\n\n {:?}", token))?;
                    //}
                    let notification = PublishDiagnosticsNotification {
                        jsonrpc: String::from("2.0"),
                        method: String::from("textDocument/publishDiagnostics"),
                        params: PublishDiagnosticsParams {
                            uri: notification.params.textDocument.uri,
                            version: 1,
                            diagnostics: vec![Diagnostic {
                                range: Range {
                                    start: Position { line: 0, character: 0 },
                                    end: Position { line: 0, character: 1 },
                                },
                                severity: 3,
                                message: String::from("your custom dod language server is running"),
                            }],
                        },
                    };
                    let stream = serialize_response(notification);
                    f.write_fmt(format_args!("\n\n writing diagnostics: {}\n\n", stream))?;
                    io::stdout().write_all(stream.as_bytes())?;
                    io::stdout().flush()?;
                } else if request.method == "textDocument/didChange" {
                    let notification: DidChangeNotification = parse_change_notification(&msg);
                    f.write_fmt(format_args!("/n/nrecieved changes: {}\n\n", notification.params.contentChanges.get(0).unwrap().text))?;
                    let tokens = Tokenizer::tokenize(&notification.params.contentChanges.get(0).unwrap().text);
                    f.write(b"\nextracted tokens\n")?;
                    let mut parser = Parser::new(&tokens);
                    parser.parse();
                    f.write(b"\nfinished parsing\n")?;
                    let diagnostics = PublishDiagnosticsNotification {
                        method: String::from("textDocument/publishDiagnostics"),
                        jsonrpc: String::from("2.0"),
                        params: PublishDiagnosticsParams { 
                            uri: notification.params.textDocument.uri,
                            version: notification.params.textDocument.version,
                            diagnostics: parser.diagnostics
                        }
                    };
                    let stream = serialize_response(diagnostics);
                    f.write_fmt(format_args!("\n\n writing diagnostics: {}\n\n", stream))?;
                    io::stdout().write_all(stream.as_bytes())?;
                    io::stdout().flush()?;
                    
                } else if request.method == "textDocument/didSave" {
                    let notification: DidSaveNotification = parse_save_notification(&msg);
                    f.write_fmt(format_args!("\n\n recieved text after save {}\n\n", notification.params.text))?;
                } else {
                    f.write_fmt(format_args!("\n\nfound request: {}\n\n", request.method))?;
                }
            }
            Err(_err) => {
                f.write(b"\r\ncouldn't read message")?;
                break;
            }
        };
    }
    Ok(())
}

fn parse_content_length(header_line: String) -> usize {
    let (_, number) = header_line
        .split_once(": ")
        .expect(format!("coulnd't split header at ': ', header is {}", header_line).as_str());
    let number: usize = number
        .trim()
        .parse()
        .expect(format!("couldn't parse number: {}", number.trim()).as_str());
    return number;
}

fn read_message() -> io::Result<String> {
    let mut header_line = String::new();
    io::stdin().read_line(&mut header_line)?;
    let content_length = parse_content_length(header_line);
    let mut content = vec![0; content_length + 2];
    io::stdin().read_exact(&mut content)?;
    return Ok(String::from_utf8(content)
        .expect("couldn't convert bytes to string")
        .trim()
        .to_string());
}

fn evaluate_expr_recursive(expr: &Expr, mut env: &mut HashMap<String, f32>) -> f32 {
    let value = match expr {
        Expr::INTEGERLITERAL(n) => *n as f32,
        Expr::FLOATLITERAL(n) => *n,
        Expr::UNARY(a) => {
            if evaluate_expr_recursive(a, &mut env) == 0.0 {
                0.0
            } else {
                1.0
            }
        }
        Expr::PARENTHESIZED(a) => evaluate_expr_recursive(a, &mut env),
        Expr::BINARYEXPR(left, right, token) => match token.kind {
            TokenKind::ADD => {
                evaluate_expr_recursive(left, &mut env) + evaluate_expr_recursive(right, &mut env)
            }
            TokenKind::MUL => {
                evaluate_expr_recursive(left, &mut env) * evaluate_expr_recursive(right, &mut env)
            }
            TokenKind::SUB => {
                evaluate_expr_recursive(left, &mut env) - evaluate_expr_recursive(right, &mut env)
            }
            TokenKind::DIV => {
                evaluate_expr_recursive(left, &mut env) / evaluate_expr_recursive(right, &mut env)
            }
            TokenKind::MOD => {
                evaluate_expr_recursive(left, &mut env) % evaluate_expr_recursive(right, &mut env)
            }
            _ => {
                panic!("unknown operator")
            }
        },
        Expr::IDENTIFIER(identifier) => {
            if env.contains_key(identifier) {
                return *env.get(identifier).unwrap();
            } else {
                panic!("variable {:?} is referenced before declaration", identifier)
            }
        }
    };

    return value;
}

fn print_expr_recursive(expr: &Expr, level: i32) {
    for _ in 0..level {
        print!("--");
    }

    match expr {
        Expr::INTEGERLITERAL(n) => {
            println!("{:?}", n);
        }
        Expr::FLOATLITERAL(n) => {
            println!("{:?}", n);
        }
        Expr::IDENTIFIER(s) => {
            println!("{:?}", s);
        }
        Expr::BINARYEXPR(a, b, c) => {
            println!("{:?}", c.kind);
            print_expr_recursive(a, level + 1);
            print_expr_recursive(b, level + 1);
        }
        Expr::UNARY(a) => {
            println!("!");
            print_expr_recursive(a, level + 1);
        }
        Expr::PARENTHESIZED(a) => {
            println!("(");
            print_expr_recursive(a, level + 1);

            for _ in 0..level {
                print!("--");
            }
            println!(")");
        }
    }
}

fn print_stmt(stmt: &Stmt, level: i32) {
    for _ in 0..level {
        print!("--");
    }

    match stmt {
        Stmt::VariableDeclaration(identifier, expr) => {
            println!("let {:?} = ", identifier);
            print_expr_recursive(expr, level + 1);
        }
        Stmt::EXPR(expr) => {
            print_expr_recursive(expr, level + 1);
        }
        Stmt::IFSTATEMENT(expr, stmts) => {
            println!("if (");
            print_expr_recursive(expr, level + 1);
            for _ in 0..level {
                print!("--");
            }
            println!("){{");
            for stmt in stmts {
                print_stmt(stmt, level + 1);
                println!("___________");
            }
            for _ in 0..level {
                print!("--");
            }
            println!("}}");
        },
        Stmt::EOF => {
            println!("None Variant");
        }
    }
}
