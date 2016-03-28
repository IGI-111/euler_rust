fn largest_palindrome(digit_count: u32) -> Option<u32> {
    let mut palindromes = Vec::new();
    for i in 1..10u32.pow(digit_count+1) {
        for j in 1..i {
            let num = i * j;
            if is_palindrome(num) {
                palindromes.push(num);
            }
        }
    }
    palindromes.into_iter().max()
}

fn is_palindrome(num: u32) -> bool {
    let length = (num as f32).log10().ceil() as u32;
    for i in 0..length / 2 as u32 {
        if nth_digit(num, 1 + i) != nth_digit(num, length - i) {
            return false;
        }
    }
    true
}

fn nth_digit(number: u32, n: u32) -> u32 {
    (number / 10u32.pow(n - 1)) % 10
}

pub fn main() {
    println!("{:?}", largest_palindrome(2));
}
