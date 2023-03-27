use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
pub struct Number {
    pub sign: bool,
    pub value: Vec<usize>,
}

impl Add for Number {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        // true is sub, false is add
        let op = self.sign ^ rhs.sign;
        let sign = self.sign;

        let mut i = 0;
        if !op {
            let mut carry = false;
            loop {
                if let Some(value) = self.value.get_mut(i) {
                    if let Some(rhs_value) = rhs.value.get(i) {
                        (*value, carry) = value.carrying_add(*rhs_value, carry);
                    } else if carry {
                        (*value, carry) = value.carrying_add(0, carry);
                    }
                } else if let Some(value) = rhs.value.get(i) {
                    let new_value;
                    (new_value, carry) = value.carrying_add(0, carry);
                    self.value.push(new_value);
                } else {
                    self.value.push(1);
                    break;
                }

                if i >= rhs.value.len() && !carry {
                    break;
                } else {
                    i += 1;
                }
            }
        } else {
            let mut borrow = false;
            if self.value.len() < rhs.value.len()
                || (self.value.len() == rhs.value.len()
                    || self.value.last().unwrap() < rhs.value.last().unwrap())
            {
                std::mem::swap(&mut self, &mut rhs);
                self.sign = sign ^ true;
            }
            loop {
                if let Some(value) = self.value.get_mut(i) {
                    if let Some(rhs_value) = rhs.value.get(i) {
                        (*value, borrow) = value.borrowing_sub(*rhs_value, borrow);
                    } else if borrow {
                        (*value, borrow) = value.borrowing_sub(0, borrow);
                    }
                } else if let Some(value) = rhs.value.get(i) {
                    let new_value;
                    (new_value, borrow) = value.borrowing_sub(0, borrow);
                    self.value.push(new_value);
                } else {
                    self.value.push(1);
                    break;
                }

                if i >= rhs.value.len() && !borrow {
                    break;
                } else {
                    i += 1;
                }
            }
        }
        self
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(mut self, mut rhs: Self) -> Self::Output {
        // true is sub, false is add
        let op = self.sign ^ rhs.sign;
        let sign = self.sign;

        let mut i = 0;
        if op {
            let mut carry = false;
            loop {
                if let Some(value) = self.value.get_mut(i) {
                    if let Some(rhs_value) = rhs.value.get(i) {
                        (*value, carry) = value.carrying_add(*rhs_value, carry);
                    } else if carry {
                        (*value, carry) = value.carrying_add(0, carry);
                    }
                } else if let Some(value) = rhs.value.get(i) {
                    let new_value;
                    (new_value, carry) = value.carrying_add(0, carry);
                    self.value.push(new_value);
                } else {
                    self.value.push(1);
                    break;
                }

                if i >= rhs.value.len() && !carry {
                    break;
                } else {
                    i += 1;
                }
            }
        } else {
            let mut borrow = false;
            if self.value.len() < rhs.value.len()
                || (self.value.len() == rhs.value.len()
                    || self.value.last().unwrap() < rhs.value.last().unwrap())
            {
                std::mem::swap(&mut self, &mut rhs);
                self.sign = sign ^ true;
            }
            loop {
                if let Some(value) = self.value.get_mut(i) {
                    if let Some(rhs_value) = rhs.value.get(i) {
                        (*value, borrow) = value.borrowing_sub(*rhs_value, borrow);
                    } else if borrow {
                        (*value, borrow) = value.borrowing_sub(0, borrow);
                    }
                } else if let Some(value) = rhs.value.get(i) {
                    let new_value;
                    (new_value, borrow) = value.borrowing_sub(0, borrow);
                    self.value.push(new_value);
                } else {
                    self.value.push(1);
                    break;
                }

                if i >= rhs.value.len() && !borrow {
                    break;
                } else {
                    i += 1;
                }
            }
        }
        self
    }
}
