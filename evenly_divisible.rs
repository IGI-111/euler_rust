fn evenly_dividable(upper_bound: i32) -> i32 {
    let mut num = upper_bound+1;
    loop{
        if (1..upper_bound+1).all(|a| num % a == 0) {
            return num;
        } else { num += 1; }
    }
}

pub fn main() {
    let res1 = evenly_dividable(10);
    println!("{}", res1);
    let res2 = evenly_dividable(20);
    println!("{}", res2);
}
