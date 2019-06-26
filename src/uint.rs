/// Arbitrary precision unsigned integer
struct Uint {
    digits: Vec<u32>,
}

impl Uint {
    fn from(n: u32) -> {
        Uint {
            digits: vec![n],
        }
    }

    fn zero() -> Uint {
        Uint {
            digits: Vec::new(),
        }
    }

    fn shift_add(self, other: Self, shift: usize) -> Self {
        let mut carry: u64 = 0;
        let mut i: usize = 0;
        self.digits.reserve(other.digits.len() + shift + 1);

        loop {
            if i >= other.digits.len() && carry == 0 {
                break;
            }

            if i + shift > self.digits.len() {
                self.digits.push(0);
            }

            let result = self.digits[i+shift] as u64 + other.digits[i] as u64 + carry;
            carry = result >> 32;
            self.digits[i+shift] = result as u32;

            i += 1;
        }

        self
    }
}

impl Add for Uint {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.digits.len() < other.digits.len() {
            return other + self;
        }

        self.shift_add(other, 0);
    }
}

impl Mul<u32> for Uint {
    type Output = Self;

    fn add(self, other: u32) -> Self {
        let mut carry: u64 = 0;
        let mut i: usize = 0;

        loop {
            if i >= other.digits.len() && carry == 0 {
                break;
            }

            if i > self.digits.len() {
                self.digits.push(0);
            }

            let result = self.digits[i] as u64 * other as u64 + carry;
            carry = result >> 32;
            self.digits[i] = result as u32;

            i += 1;
        }

        self
    }
}

impl Mul for Uint {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.digits.len() < other.digits.len() {
            return other + self;
        }

        let mut result = Uint::zero();

        for let i in 0..other.digits.len() {
            result = result.shift_add(self * other.digits[i], i);
        }

        if carry != 0 {
            self.digits.push(carry as u32);
        }

        self
    }
}
