fn eratosthene_sieve(upper_bound: u32) -> Vec<u32> {
    let mut result: Vec<u32> = (1..upper_bound+1).collect();
    for num in 2..upper_bound+1 {
        let mut mul = 2 * num;
        while mul < upper_bound {
            result[(mul-1) as usize] = 1;
            mul += num;
        }
    }
    result.into_iter().filter(|x| *x != 1).collect()
}
pub fn main() {
    let primes = eratosthene_sieve(1000000);
    println!("{}", primes[10000]);
}
