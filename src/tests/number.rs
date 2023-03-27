use crate::runtime::number::Number;

macro_rules! number {
    (-, $($i:expr),*) => {
        Number {
            sign: false,
            value: vec![$($i), *]
        }
    };
    (+, $($i:expr),*) => {
        Number {
            sign: true,
            value: vec![$($i), *]
        }
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn overflow_handling_with_lhs() {
        use crate::runtime::number::Number;

        let lhs = Number {
            sign: true,
            value: vec![usize::MAX, usize::MAX],
        };

        let rhs = Number {
            sign: true,
            value: vec![1],
        };

        let n = Number {
            sign: true,
            value: vec![0, 0, 1],
        };

        assert_eq!(lhs + rhs, n);
    }

    #[test]
    fn overflow_handling_with_rhs() {
        use crate::runtime::number::Number;

        let lhs = Number {
            sign: true,
            value: vec![1],
        };

        let rhs = Number {
            sign: true,
            value: vec![usize::MAX, usize::MAX],
        };

        let n = Number {
            sign: true,
            value: vec![0, 0, 1],
        };

        assert_eq!(lhs + rhs, n);
    }

    #[test]
    fn addition_negative_greater_rhs() {
        use crate::runtime::number::Number;

        let lhs = number!(+, 1);
        let rhs = number!(-, usize::MAX, usize::MAX);

        let n = number!(-, usize::MAX-1, usize::MAX);

        assert_eq!(lhs + rhs, n);
    }

    #[test]
    fn addition_negative_greater_lhs() {
        use crate::runtime::number::Number;

        let lhs = Number {
            sign: false,
            value: vec![usize::MAX, usize::MAX],
        };

        let rhs = Number {
            sign: true,
            value: vec![1],
        };

        let n = Number {
            sign: false,
            value: vec![usize::MAX - 1, usize::MAX],
        };

        assert_eq!(lhs + rhs, n);
    }
}
