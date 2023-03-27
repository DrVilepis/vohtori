pub mod error;
pub mod number;
pub mod operation;

use std::collections::HashMap;

use crate::{
    library,
    parser::{Expr, Number},
};

use self::operation::{operate, Modifier, Operation, Operator};

pub struct Runtime {
    variables: HashMap<String, Value>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn push_var(&mut self, name: &str, val: Value) {
        self.variables.insert(name.to_owned(), val);
    }

    pub fn eval_expr(&mut self, expr: Expr, body_args: Option<&Array>) -> Value {
        match expr {
            Expr::Binary(op, lhs, rhs) => {
                let op = Operation {
                    operator: Operator::from_str(&op.name).unwrap(),
                    modifier: op.modifiers,
                };
                let lhs = self.eval_expr(*lhs, body_args);
                let rhs = self.eval_expr(*rhs, body_args);

                apply(op, lhs, rhs)
            }
            Expr::Unary(op, val) => {
                let op = Operation {
                    operator: Operator::from_str(&op.name).unwrap(),
                    modifier: op.modifiers,
                };
                let val = self.eval_expr(*val, body_args);

                apply_unary(op, val)
            }
            Expr::Variable(var) => self.variables.get(&var.name).unwrap().clone(),
            Expr::Number(i) => Value::Number(i),
            Expr::Array(arr) => Value::Array(Array {
                value: arr
                    .into_iter()
                    .map(|e| self.eval_expr(e, body_args))
                    .collect(),
            }),
            Expr::Call(function, args) => match *function {
                Expr::Function(ident) => {
                    let args = self.eval_expr(*args, body_args);

                    match ident.name.as_str() {
                        "idx" => library::index(args),
                        _ => panic!()
                    }
                }
                Expr::Lambda(lambda) => {
                    let args = self.eval_expr(*args, body_args);
                    self.eval_expr(*lambda.body, Some(&args.into_array()))
                }
                _ => {
                    todo!()
                }
            },
            Expr::Argument(arg) => {
                if let Some(arg_env) = body_args {
                    arg_env.value[arg.index].clone()
                } else {
                    panic!()
                }
            }
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Array {
    pub value: Vec<Value>,
}

#[derive(Clone, Debug)]
pub enum Value {
    Array(Array),
    Number(Number),
}

impl Value {
    fn into_array(self) -> Array {
        if let Value::Array(array) = self {
            array
        } else {
            Array { value: vec![self] }
        }
    }
}

fn apply(op: Operation, lhs: Value, rhs: Value) -> Value {
    match (lhs, rhs) {
        (Value::Array(lhs), Value::Number(rhs)) => {
            let output = lhs
                .value
                .into_iter()
                .map(|v| apply(op, v, Value::Number(rhs.clone())))
                .collect();

            Value::Array(Array { value: output })
        }
        (Value::Number(lhs), Value::Array(rhs)) => {
            let output = rhs
                .value
                .into_iter()
                .map(|v| apply(op, Value::Number(lhs.clone()), v))
                .collect();

            Value::Array(Array { value: output })
        }
        (Value::Array(lhs), Value::Array(rhs)) => {
            let output = if op.modifier.contains(Modifier::Table) {
                lhs.value
                    .into_iter()
                    .map(|lhs| {
                        Value::Array(Array {
                            value: rhs
                                .value
                                .iter()
                                .map(|rhs| apply(op, lhs.clone(), rhs.clone()))
                                .collect(),
                        })
                    })
                    .collect()
            } else {
                lhs.value
                    .into_iter()
                    .zip(rhs.value.into_iter())
                    .map(|(lhs, rhs)| apply(op, lhs, rhs))
                    .collect()
            };

            Value::Array(Array { value: output })
        }
        (Value::Number(lhs), Value::Number(rhs)) => {
            if op.modifier.contains(Modifier::Flip) {
                operate(op.operator, rhs, lhs)
            } else {
                operate(op.operator, lhs, rhs)
            }
        }
    }
}

fn apply_unary(op: Operation, val: Value) -> Value {
    match val {
        Value::Array(mut arr) => {
            let mut first = arr.value.pop().unwrap();

            for v in arr.value {
                first = apply(op, first, v);
            }

            first
        }
        Value::Number(_) => val,
    }
}
