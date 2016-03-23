use std::collections::HashSet;

fn multiples(num: u32, upper_bound: u32) -> HashSet<u32> {
    let mut i = num;
    let mut result = HashSet::new();
    while i < upper_bound {
        result.insert(i);
        i += num;
    }
    result
}

fn print_multiples(nums: &[u32], upper_bound: u32) {
    let mut muls = HashSet::new();
    for num in nums.iter() {
        let num_muls = multiples(*num, upper_bound);
        muls = muls.union(&num_muls).cloned().collect();
    }
    let sum = muls.clone().into_iter().fold(0, |a, b| a + b);
    println!("Multiples of {:?} below {}: {:?}, summed: {}",
             nums,
             upper_bound,
             muls,
             sum);
}

pub fn main() {
    print_multiples(&[3, 5], 10);
    print_multiples(&[3, 5], 1000);
}
