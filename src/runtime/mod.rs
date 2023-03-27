pub mod error;
pub mod number;
pub mod operation;

use std::collections::HashMap;

use crate::parser::{Expr, Number};

use self::operation::{operate, Modifier, Operation, Operator};

pub struct Runtime {
    variables: HashMap<String, Array>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn eval_expr(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Binary(op, lhs, rhs) => {
                let op = Operation {
                    operator: Operator::from_str(&op.name).unwrap(),
                    modifier: op.modifiers,
                };
                let lhs = self.eval_expr(*lhs);
                let rhs = self.eval_expr(*rhs);

                apply(op, lhs, rhs)
            }
            Expr::Unary(op, val) => {
                let op = Operation {
                    operator: Operator::from_str(&op.name).unwrap(),
                    modifier: op.modifiers,
                };
                let val = self.eval_expr(*val);

                apply_unary(op, val)
            }
            Expr::Variable(var) => {
                todo!()
            }
            Expr::Number(i) => Value::Number(i),
            Expr::Array(arr) => Value::Array(Array {
                value: arr.into_iter().map(|e| self.eval_expr(e)).collect(),
            }),
            Expr::Function(ident, args) => {
                let args = self.eval_expr(*args);
                if ident.name == "idx" {
                    if let Value::Array(mut arr) = args {
                        arr.value.iter_mut().enumerate().for_each(|(i, n)| {
                            match n {
                                Value::Array(arr) => (),
                                Value::Number(num) => {
                                    if num.value != 0 {
                                        num.value = i as isize;
                                    }
                                },
                            };
                        });
                        Value::Array(arr)
                    } else {
                        args
                    }
                } else {
                    todo!()
                }
            },
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
