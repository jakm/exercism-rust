#[macro_use]
extern crate lazy_static;

use std::cmp::{Ordering, PartialEq};
use std::fmt;
use std::ops::{Add, Mul, Sub};

use num_bigint::{BigInt, ToBigInt};
use num_traits::Zero;

lazy_static! {
    static ref BIG_TEN: BigInt = 10_u8.to_bigint().unwrap();
}

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Clone, Debug)]
pub struct Decimal {
    int_val: BigInt,
    scale: u32,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        if input.len() == 0 {
            return None;
        }

        let int_val = BigInt::parse_bytes(input.replace(".", "").as_bytes(), 10)?;

        let scale: u32 = if int_val.is_zero() {
            0
        } else {
            if let Some(idx) = input.find('.') {
                let n = input[idx + 1..].len();
                if n > u32::MAX as usize {
                    panic!("too big fractional part");
                }
                n as u32
            } else {
                0
            }
        };

        Some(Decimal { int_val, scale })
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        match self.scale.cmp(&other.scale) {
            Ordering::Equal => self.int_val == other.int_val,
            Ordering::Greater => {
                let v = other.int_val.clone() * &BIG_TEN.pow(self.scale - other.scale);
                self.int_val == v
            }
            Ordering::Less => {
                let v = self.int_val.clone() * &BIG_TEN.pow(other.scale - self.scale);
                v == other.int_val
            }
        }
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.scale.cmp(&other.scale) {
            Ordering::Equal => Some(self.int_val.cmp(&other.int_val)),
            Ordering::Greater => {
                let v = other.int_val.clone() * &BIG_TEN.pow(self.scale - other.scale);
                Some(self.int_val.cmp(&v))
            }
            Ordering::Less => {
                let v = self.int_val.clone() * &BIG_TEN.pow(other.scale - self.scale);
                Some(v.cmp(&other.int_val))
            }
        }
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.int_val.to_str_radix(10);
        let scale = self.scale as usize;
        if s.len() - scale > 0 {
            write!(f, "{}", &s[..s.len() - scale])?;
        } else {
            write!(f, "0")?;
        }
        if scale > 0 {
            write!(f, ".{}", &s[s.len() - scale..])?;
        }

        Ok(())
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self.scale.cmp(&rhs.scale) {
            Ordering::Equal => Decimal {
                int_val: self.int_val + rhs.int_val,
                scale: self.scale,
            },
            Ordering::Greater => Decimal {
                int_val: self.int_val + rhs.int_val * &BIG_TEN.pow(self.scale - rhs.scale),
                scale: self.scale,
            },
            Ordering::Less => Decimal {
                int_val: self.int_val * &BIG_TEN.pow(rhs.scale - self.scale) + rhs.int_val,
                scale: rhs.scale,
            },
        }
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self.scale.cmp(&rhs.scale) {
            Ordering::Equal => Decimal {
                int_val: self.int_val - rhs.int_val,
                scale: self.scale,
            },
            Ordering::Greater => Decimal {
                int_val: self.int_val - rhs.int_val * &BIG_TEN.pow(self.scale - rhs.scale),
                scale: self.scale,
            },
            Ordering::Less => Decimal {
                int_val: self.int_val * &BIG_TEN.pow(rhs.scale - self.scale) - rhs.int_val,
                scale: rhs.scale,
            },
        }
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Decimal {
            int_val: self.int_val * rhs.int_val,
            scale: self.scale + rhs.scale,
        }
    }
}
