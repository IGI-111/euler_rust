use std::collections::{HashSet, HashMap};

fn fermat_factor(n: u64) -> u64{
    let mut a = (n as f64).sqrt().ceil();
    let mut b2 = a * a - n as f64;
    while !is_square(b2) {
        a += 1.;
        b2 = a * a - n as f64;
    }
    (a - b2.sqrt()) as u64
}

fn is_square(n: f64) -> bool {
    n.sqrt() == n.sqrt().round()
}

fn prime_factors(n: u64) -> HashSet<u64> {
    let mut factors = HashSet::new();
    factors.insert(n);

    loop {
        let mut new_factors = HashMap::new();
        for old in factors.iter() {
            let new = fermat_factor(*old);
            if new > 1 {
                new_factors.insert(old.clone(), new);
            }
        }
        for (old, new) in new_factors.iter() {
            factors.remove(old);
            factors.insert(*new);
            factors.insert(*old / *new);
        }
        if new_factors.is_empty() {
            break;
        }
    }

    factors
}

pub fn main() {
    println!("{:?}", prime_factors(600851475143));
}
