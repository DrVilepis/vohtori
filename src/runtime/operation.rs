use crate::parser::Number;

use super::{Array, Value};

use bitflags::bitflags;

bitflags! {
    #[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Modifier: u32 {
        const Flip  = 0b00000001;
        const Table = 0b00000010;
    }
}

#[derive(Copy, Clone)]
pub struct Operation {
    pub operator: Operator,
    pub modifier: Modifier,
}

#[derive(Copy, Clone)]
pub enum Operator {
    Add,
    Sub,
    Div,
    Mul,
    Mod,
    Range,
    Eq,
    Or,
}

impl Operator {
    pub fn from_str(value: &str) -> Option<Self> {
        Some(match value {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            "*" => Operator::Mul,
            "/" => Operator::Div,
            "%" => Operator::Mod,
            ".." => Operator::Range,
            "==" => Operator::Eq,
            "||" => Operator::Or,
            _ => return None,
        })
    }
}

pub fn operate(op: Operator, lhs: Number, rhs: Number) -> Value {
    match op {
        Operator::Add => Value::Number(Number {
            value: lhs.value + rhs.value,
        }),
        Operator::Sub => Value::Number(Number {
            value: lhs.value - rhs.value,
        }),
        Operator::Div => Value::Number(Number {
            value: lhs.value / rhs.value,
        }),
        Operator::Mul => Value::Number(Number {
            value: lhs.value * rhs.value,
        }),
        Operator::Mod => Value::Number(Number {
            value: lhs.value % rhs.value,
        }),
        Operator::Range => Value::Array(Array {
            value: (lhs.value..=rhs.value)
                .map(|n| Value::Number(Number { value: n }))
                .collect(),
        }),
        Operator::Eq => Value::Number(Number {
            value: if lhs.value == rhs.value { 1 } else { 0 },
        }),
        Operator::Or => Value::Number(Number {
            value: if lhs.value != 0 || rhs.value != 0 {
                1
            } else {
                0
            },
        }),
        _ => todo!(),
    }
}
