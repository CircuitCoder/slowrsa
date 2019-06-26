use std::ops::*;
use std::cmp::*;

/// Arbitrary precision unsigned integer
#[derive(Clone, Debug, PartialEq, Eq)]
struct Uint {
    digits: Vec<u64>,
}

impl Uint {
    fn from(n: u64) -> Self {
        Uint {
            digits: vec![n],
        }
    }

    fn zero() -> Uint {
        Uint {
            digits: Vec::new(),
        }
    }

    fn trim(mut self) -> Self {
        while self.digits.len() > 0 && self.digits[self.digits.len()-1] == 0 {
            self.digits.pop();
        }

        self
    }

    fn shift_add(mut self, other: Self, shift: usize) -> Self {
        // println!("SHIFT_ADD: {:?} + {:?} << {}", self, other, shift);
        let mut carry: u128 = 0;
        let mut i: usize = 0;
        if other.digits.len() + shift + 1 > self.digits.len() {
            self.digits.reserve(other.digits.len() + shift + 1 - self.digits.len());
        }

        loop {
            if i >= other.digits.len() && carry == 0 {
                break;
            }

            if i + shift >= self.digits.len() {
                self.digits.push(0);
            }

            let result = self.digits[i+shift] as u128 + other.digits[i] as u128 + carry;
            carry = result >> 64;
            self.digits[i+shift] = result as u64;

            i += 1;
        }

        self.trim()
    }

    fn get_bit(&self, bit: usize) -> bool {
        let outer = bit / 64;
        let inner = bit % 64;
        if outer >= self.digits.len() {
            false
        } else {
            self.digits[outer].overflowing_shr(inner as u32).0 & 1 != 0
        }
    }

    fn set_bit(&mut self, bit: usize, cont: bool) {
        let outer = bit / 64;
        let inner = bit % 64;
        if outer >= self.digits.len() {
            self.digits.resize_with(outer+1, Default::default);
        }

        self.digits[outer] &= !(1u64 << inner);
        if cont {
            self.digits[outer] |= 1u64 << inner;
        }
    }

    fn divrem(&self, other: &Uint) -> (Uint, Uint) {
        if self < other {
            return (Uint::zero(), other.clone());
        }

        let mut reminder = Uint::zero();
        let mut quotient = Uint::zero();

        for i in (0..self.digits.len()*64).rev() {
            println!("DIVINNER: {}, {:?} - {:?}", i, reminder, quotient);
            reminder = reminder << 1;
            reminder.set_bit(0, self.get_bit(i));
            
            if reminder >= *other {
                reminder = reminder - other.clone();
                quotient.set_bit(i, true);
            }
        }

        (quotient, reminder)
    }

    fn mod_sub(mut self, mut other: Uint, m: &Uint) -> Uint {
        other = other.divrem(m).1;

        if self < other {
            self = self + m.clone();
        }

        self = self - other;
        self.divrem(m).1
    }

    fn mod_pow(mut self, p: &Uint, m: &Uint) -> Uint {
        let mut result = self.clone();
        for i in 0..p.top_bit() {
            result = result.clone() * result;
            if p.get_bit(i) {
                result = result * self.clone()
            }

            result = result.divrem(m).1;
        }

        result
    }

    fn top_bit(&self) -> usize {
        self.digits.len() * 64 - self.digits[self.digits.len() - 1].leading_zeros() as usize
    }
}

impl Add for Uint {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.digits.len() < other.digits.len() {
            return other + self;
        }

        self.shift_add(other, 0)
    }
}

impl Sub for Uint {
    type Output = Self;

    /// Non wrapping
    fn sub(mut self, other: Self) -> Self {
        if self.digits.len() < other.digits.len() {
            return Uint::zero();
        }

        let mut carry: u128 = 0;
        let mut i: usize = 0;
        loop {
            if i >= other.digits.len() && carry == 0 {
                break;
            }

            if i < other.digits.len() {
                carry += other.digits[i] as u128;
            }

            if i >= self.digits.len() {
                return Uint::zero();
            }

            if carry > self.digits[i] as u128 {
                self.digits[i] = ((1u128 << 64) + self.digits[i] as u128 - carry) as u64;
                carry = 1;
            } else {
                self.digits[i] -= carry as u64;
                carry = 0;
            }

            i += 1;
        }

        self.trim()
    }
}

impl Mul<u64> for Uint {
    type Output = Self;

    fn mul(mut self, other: u64) -> Self {
        let mut carry: u128 = 0;
        let mut i: usize = 0;

        loop {
            if i != 0 && carry == 0 {
                break;
            }

            if i >= self.digits.len() {
                self.digits.push(0);
            }

            let result = self.digits[i] as u128 * other as u128 + carry;
            carry = result >> 64;
            self.digits[i] = result as u64;

            i += 1;
        }

        println!("INTER: {:?}", self);
        self.trim()
    }
}

impl Mul for Uint {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.digits.len() < other.digits.len() {
            return other * self;
        }

        let mut result = Uint::zero();

        for i in 0..other.digits.len() {
            result = result.shift_add(self.clone() * other.digits[i], i);
        }

        result.trim()
    }
}

impl Shl<usize> for Uint {
    type Output = Self;

    fn shl(self, other: usize) -> Self {
        // Move first
        let outer = other / 64;
        let inner = other % 64;

        let mut result = Vec::with_capacity(outer + 1 + self.digits.len());
        result.resize_with(outer + 1 + self.digits.len(), Default::default);

        for i in 0..self.digits.len() {
            result[i + outer] |= self.digits[i].overflowing_shl(inner as u32).0;
            if inner > 0 {
                result[i + outer + 1] |= self.digits[i].overflowing_shr((64 - inner) as u32).0;
            }
        }

        (Uint {
            digits: result,
        }).trim()
    }
}

impl PartialOrd for Uint {
    fn partial_cmp(&self, other: &Uint) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Uint {
    fn cmp(&self, other: &Uint) -> Ordering {
        if self.digits.len() == other.digits.len() {
            for i in (0..self.digits.len()).rev() {
                if self.digits[i] != other.digits[i] {
                    return self.digits[i].cmp(&other.digits[i]);
                }
            }
            return Ordering::Equal;
        } else {
            self.digits.len().cmp(&other.digits.len())
        }
    }
}

impl Div for Uint {
    type Output = Self;

    fn div(self, other: Uint) -> Self {
        self.divrem(&other).0
    }
}

impl Rem for Uint {
    type Output = Self;

    fn rem(self, other: Uint) -> Self {
        self.divrem(&other).1
    }
}

#[test]
fn test_add() {
    let a = Uint::from(100000000000000000u64);
    let b = Uint::from(200000000000000000u64);
    
    println!("ADD: {:?}", a+b);
}

#[test]
fn test_mul() {
    let a = Uint::from(100000000000000000u64);
    let b = Uint::from(200000000000000000u64);
    
    println!("MUL: {:?}", a*b);
}

#[test]
fn test_sub() {
    let a = Uint::from(100000000000000000u64);
    let b = Uint::from(200000000000000000u64);
    let a2 = Uint::from(100000000000000000u64);
    let b2 = Uint::from(300000000000000000u64);

    let c = a*b;
    let c2 = a2*b2;
    println!("SUB1: {:?}", c);
    println!("SUB2: {:?}", c2);
    
    println!("SUB: {:?}", c2 - c);
}

#[test]
fn test_shl() {
    let a = Uint::from(100000000000000000u64);
    let b = Uint::from(200000000000000000u64);
    
    println!("{:?}", a << 100);
    println!("{:?}", b << 99);
}

#[test]
fn test_cmp() {
    let a = Uint::from(100000000000000000u64);
    let b = Uint::from(200000000000000000u64);
    let c = Uint::from(300000000000000000u64);
    
    println!("{:?}", a.clone() * b < a * c);
}

#[test]
fn test_divrem() {
    let a = Uint::from(100);
    let b = Uint::from(17);
    
    println!("DIVREM: {:?}", a.divrem(&b));
}

#[test]
fn test_pow() {
    let a = Uint::from(100);
    let b = Uint::from(17);
    let c = Uint::from(48);
    
    println!("MODPOW: {:?}", a.mod_pow(&b, &c));
}
