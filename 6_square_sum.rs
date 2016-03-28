pub fn main() {
    fn square(x: u32) -> u32 { x * x }
    let sum_of_squares = (1..101).map(square).fold(0, |a, b| a + b);
    let square_of_sum = square((1..101).fold(0, |a, b| a + b));
    println!("{} - {} = {}",
             square_of_sum,
             sum_of_squares,
             square_of_sum - sum_of_squares);
}
