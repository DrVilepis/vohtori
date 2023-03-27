use crate::runtime::{Value, Array};

pub fn index(arg: Value) -> Value {
    if let Value::Array(mut arr) = arg {
        index_arr(&mut arr);

        Value::Array(arr)
    } else {
        arg
    }
}

fn index_arr(arg: &mut Array) {
    arg.value.iter_mut().enumerate().for_each(|(i, n)| {
        match n {
            Value::Array(arr) => {
                index_arr(arr)
            },
            Value::Number(num) => {
                if num.value != 0 {
                    num.value = i as isize;
                }
            }
        };
    });
}
