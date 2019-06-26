use ramp::Int;
use rand;

use ramp::RandomInt;

pub struct PrivKey {
    n: Int,
    d: Int,
}

pub struct PubKey {
    n: Int,
    e: Int,
}

impl PrivKey {
    pub fn print(&self) {
        println!("n: {:X}", self.n);
        println!("d: {:X}", self.d);
    }

    pub fn from_str(n: &str, d: &str) -> Result<PrivKey, ramp::int::ParseIntError> {
        Ok(PrivKey {
            n: Int::from_str_radix(n, 16)?,
            d: Int::from_str_radix(d, 16)?,
        })
    }

    pub fn decrypt(&self, c: &str) -> Result<String, ramp::int::ParseIntError> {
        let cint = Int::from_str_radix(c, 16)?;
        let m = cint.pow_mod(&self.d, &self.n);
        Ok(format!("{:X}", m))
    }
}

impl PubKey {
    pub fn print(&self) {
        println!("n: {:X}", self.n);
        println!("e: {:X}", self.e);
    }

    pub fn from_str(n: &str, e: &str) -> Result<PubKey, ramp::int::ParseIntError> {
        Ok(PubKey {
            n: Int::from_str_radix(n, 16)?,
            e: Int::from_str_radix(e, 16)?,
        })
    }

    pub fn encrypt(&self, m: &str) -> Result<String, ramp::int::ParseIntError> {

        let mint = Int::from_str_radix(m, 16)?;
        let c = mint.pow_mod(&self.e, &self.n);
        Ok(format!("{:X}", c))
    }
}

fn modinv(m: Int, n: Int) -> Int {
    let mut s = (Int::from(0), Int::from(1));
    let mut r = (n.clone(), m);

    while r.0 != 0 {
        let q = r.1.clone() / r.0.clone();

        r = (r.1 - q.clone() * r.0.clone(), r.0);
        s = (s.1 - q * s.0.clone(), s.0);
    }

    let mut result = s.1 % n.clone();
    while result < 0 {
        result += n.clone();
    }

    result
}

pub fn keygen() -> (PrivKey, PubKey) {
    let p = rand_prime(1024, 6);
    let q = rand_prime(1024, 6);
    println!("{}", p);
    println!("{}", q);

    let e = Int::from(65537);
    let n = p.clone() * q.clone();
    let phi_n: Int = (p-1) * (q-1);
    let d = modinv(e.clone(), phi_n); 
    
    let priv_key = PrivKey {
        d,
        n: n.clone(),
    };

    let pub_key = PubKey {
        e,
        n,
    };

    return (priv_key, pub_key);
}

const SMALL_PRIMES: [u64; 169] = [3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43,
                            47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101,
                            103, 107, 109, 113, 127, 131, 137, 139, 149, 151,
                            157, 163, 167, 173, 179, 181, 191, 193, 197, 199,
                            211, 223, 227, 229, 233, 239, 241, 251, 257, 263,
                            269, 271, 277, 281, 283, 293, 307, 311, 313, 317,
                            331, 337, 347, 349, 353, 359, 367, 373, 379, 383,
                            389, 397, 401, 409, 419, 421, 431, 433, 439, 443,
                            449, 457, 461, 463, 467, 479, 487, 491, 499, 503,
                            509, 521, 523, 541, 547, 557, 563, 569, 571, 577,
                            587, 593, 599, 601, 607, 613, 617, 619, 631, 641,
                            643, 647, 653, 659, 661, 673, 677, 683, 691, 701,
                            709, 719, 727, 733, 739, 743, 751, 757, 761, 769,
                            773, 787, 797, 809, 811, 821, 823, 827, 829, 839,
                            853, 857, 859, 863, 877, 881, 883, 887, 907, 911,
                            919, 929, 937, 941, 947, 953, 967, 971, 977, 983,
                            991, 997, 1009, 1013];

fn rand_prime(digits: usize, t: usize) -> Int {
    let mut counter = 0;
    'gen: loop {
        let mut rng = rand::thread_rng();
        let mut num = rng.gen_uint(digits-1);
        num.set_bit(0, true);
        num.set_bit((digits-1) as u32, true);

        for small_prime in SMALL_PRIMES.iter() {
            if num.clone() % *small_prime == 0 {
                continue 'gen;
            }
        }

        let dec: ramp::Int = num.clone() - 1;
        let k = dec.trailing_zeros() as usize;
        let d = dec.clone() >> k;
        let two = ramp::Int::from(2);

        'outer: for _ in 0..t {
            let a = loop {
                let r = rng.gen_uint_below(&dec);
                if r < 2 { continue; }
                break r;
            };

            let mut y = a.pow_mod(&d, &num);

            if y == 1 || y == dec {
                continue
            }

            for _ in 0..k {
                y = y.pow_mod(&two, &num);
                if y == dec {
                    continue 'outer;
                }
            }

            counter += 1;
            if counter % 10 == 0 {
                println!("{}", counter);
            }
            continue 'gen;
        }

        break num;
    }
}
