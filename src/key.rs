use rand;
use crate::uint::Uint;
use std::io::Write;

pub struct PrivKey {
    n: Uint,
    d: Uint,
}

pub struct PubKey {
    n: Uint,
    e: Uint,
}

impl PrivKey {
    pub fn print(&self) {
        println!("n: {}", self.n.to_hex());
        println!("d: {}", self.d.to_hex());
    }

    pub fn from_str(n: &str, d: &str) -> Result<PrivKey, ()> {
        Ok(PrivKey {
            n: Uint::from_str(n),
            d: Uint::from_str(d),
        })
    }

    pub fn decrypt(&self, c: &str) -> Result<String, ()> {
        let cint = Uint::from_str(c);
        let m = cint.mod_pow(&self.d, &self.n);
        Ok(format!("{}", m.to_hex()))
    }
}

impl PubKey {
    pub fn print(&self) {
        println!("n: {}", self.n.to_hex());
        println!("e: {}", self.e.to_hex());
    }

    pub fn from_str(n: &str, e: &str) -> Result<PubKey, ()> {
        Ok(PubKey {
            n: Uint::from_str(n),
            e: Uint::from_str(e),
        })
    }

    pub fn encrypt(&self, m: &str) -> Result<String, ()> {
        let mint = Uint::from_str(m);
        let c = mint.mod_pow(&self.e, &self.n);
        Ok(format!("{}", c.to_hex()))
    }
}

fn modinv(m: Uint, n: Uint) -> Uint {
    let mut s = (Uint::zero(), Uint::from(1));
    let mut r = (n.clone(), m);

    while r.0 != Uint::zero() {
        let q = r.1.clone() / r.0.clone();

        r = (r.1.mod_sub(q.clone() * r.0.clone(), &n), r.0);
        s = (s.1.mod_sub(q * s.0.clone(), &n), s.0);
    }

    let mut result = s.1 % n.clone();
    while result < Uint::zero() {
        result = result + n.clone();
    }

    result
}

pub fn keygen() -> (PrivKey, PubKey) {
    let p = rand_prime(1024, 6);
    let q = rand_prime(1024, 6);

    let e = Uint::from(65537);
    let n = p.clone() * q.clone();
    let phi_n: Uint = (p-Uint::from(1)) * (q-Uint::from(1));
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

fn rand_prime(digits: usize, t: usize) -> Uint {
    let mut counter = 0;
    'gen: loop {
        // let mut num = rng.gen_uint(digits-1);
        let mut num = Uint::rand(digits / 64);
        num.set_bit(0, true);
        num.set_bit(digits-1, true);

        for small_prime in SMALL_PRIMES.iter() {
            if num.clone() % Uint::from(*small_prime) == Uint::zero() {
                continue 'gen;
            }
        }

        let dec: Uint = num.clone() - Uint::from(1);
        let mut k = 0;
        while !dec.get_bit(k) {
            k += 1;
        }
        let d = dec.clone() >> k;

        'outer: for _ in 0..t {
            let a = loop {
                let r = Uint::rand(digits/64);
                if r < Uint::from(2) { continue; }
                if r >= dec { continue; }
                break r;
            };

            let mut y = a.mod_pow(&d, &num);

            if y == Uint::from(1) || y == dec {
                continue
            }

            for _ in 0..k {
                y = y.mod_pow(&Uint::from(2), &num);
                // println!("{:?}", dec.clone() - y.clone());
                // println!("{:?}", y.clone() - dec.clone());
                if y == dec {
                    continue 'outer;
                }
            }

            counter += 1;
            if counter % 10 == 0 {
                print!(".");
                std::io::stdout().flush().unwrap();
            }
            continue 'gen;
        }

        println!("{}", counter);
        break num;
    }
}
