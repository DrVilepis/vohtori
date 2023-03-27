#![feature(result_option_inspect, is_some_and, box_syntax, bigint_helper_methods)]

#[cfg(test)]
mod tests;

use std::io::Read;

mod lexer;
mod parser;
mod runtime;

use lexer::SourceCursor;

use parser::{Parser, TokenStream};
use runtime::Value;

use crate::runtime::Runtime;

use clap::Parser as ClapParser;

#[derive(ClapParser, Debug)]
#[command()]
struct Args {
    #[arg(short, long)]
    lexer: bool,

    #[arg(short, long)]
    parser: bool,
}

fn main() {
    let args = Args::parse();

    let mut stdin = std::io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();

    let source = SourceCursor::new(&input);

    if args.lexer {
        eprintln!("LEXER OUTPUT:");
        let mut source = source.clone();
        while let Some(token) = source.advance_token() {
            eprintln!("{:#?}", token);
        }
        eprintln!();
    }

    let mut parser = Parser::new(TokenStream::new(source));

    let mut runtime = Runtime::new();

    let expr = parser.parse_expr();

    if args.parser {
        eprintln!("PARSER OUTPUT:");
        eprintln!("{:#?}", expr);
        eprintln!();
    }

    let value = runtime.eval_expr(expr);
    pprint(&value);
    println!("");
}

fn pprint(value: &Value) {
    match value {
        Value::Array(array) => {
            print!("[");

            let len = array.value.len();

            for i in 0..(len - 1) {
                pprint(&array.value[i]);
                print!(" ");
            }
            pprint(&array.value[len - 1]);

            print!("]");
        }
        Value::Number(number) => print!("{}", number.value),
    }
}
