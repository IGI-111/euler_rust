fn fibonacci(upper_bound: u32) -> Vec<u32> {
    let mut seq = vec![1];
    let mut next: u32 = 2;
    while next < upper_bound {
        seq.push(next);
        next = match seq.split_last() {
            None => panic!("seq is empty"),
            Some((ultimate, front)) => match front.last() {
                None => panic!("seq has only one element"),
                Some(penultimate) => penultimate + ultimate
            }
        };
    }
    seq
}
pub fn main() {
    let sum: u32 = fibonacci(4000000).iter().fold(0, |a,b|
                                                 if b % 2 == 0 {a+b}
                                                 else {a});
    println!("{:?} {}", fibonacci(4000000), sum);
}
