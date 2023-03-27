#![feature(result_option_inspect, is_some_and, bigint_helper_methods)]

#[cfg(test)]
mod tests;

mod lexer;
mod library;
mod parser;
mod runtime;

use std::{fs::File, io::{BufReader, Read}, path::PathBuf};

use lexer::SourceCursor;

use parser::{Parser, Number};
use runtime::{Value, Array};

use crate::runtime::Runtime;

use clap::Parser as ClapParser;

#[derive(ClapParser, Debug)]
#[command()]
struct Args {
    #[arg(short, long)]
    interactive: bool,

    #[arg(short, long)]
    parser: bool,
    input: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let mut stdin = std::io::stdin().lock();

    let mut runtime = Runtime::new();

    if args.interactive {
        let source = SourceCursor::new(stdin);

        let mut parser = Parser::new(source);

        while let Some(expr) = parser.parse_expr() {
            if args.parser {
                eprintln!("PARSER OUTPUT:");
                eprintln!("{:#?}", expr);
                eprintln!();
            }

            match expr {
                Ok(expr) => {
                    let value = runtime.eval_expr(expr, None);

                    print!("  = ");
                    pprint(&value);
                    println!();
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
    } else if let Some(path) = args.input {
        let file = File::open(path).unwrap();
        let source = SourceCursor::new(BufReader::new(file));

        let mut parser = Parser::new(source);

        let mut input = String::new();
        stdin.read_to_string(&mut input).unwrap();

        runtime.push_var("stdin", parse_list(input));


        while let Some(expr) = parser.parse_expr() {
            match expr {
                Ok(expr) => {
                    let value = runtime.eval_expr(expr, None);

                    pprint(&value);
                    println!();
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
    }
}

fn pprint(value: &Value) {
    match value {
        Value::Array(array) => {
            print!("[");

            let len = array.value.len();

            if len > 0 {
                for i in 0..(len - 1) {
                    pprint(&array.value[i]);
                    print!(" ");
                }

                pprint(&array.value[len - 1]);
            }

            print!("]");
        }
        Value::Number(number) => print!("{}", number.value),
    }
}

fn parse_list(input: String) -> Value {
    Value::Array(Array {
        value: input.lines().map(|l| {
            Value::Array(Array {
                value: l.split_whitespace().map(|v| Value::Number(Number {value: v.parse().unwrap() })).collect(),
            })
        }).collect(),
    })
}
