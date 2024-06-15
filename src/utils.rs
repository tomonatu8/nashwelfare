pub fn to_base(mut n: u64, k: u32) -> Vec<char> {
    if !(2..=36).contains(&k) {
        panic!("Base must be between 2 and 36");
    }

    let mut result = Vec::new();
    let digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    while n > 0 {
        let remainder = (n % k as u64) as usize;
        result.push(digits.chars().nth(remainder).unwrap());
        n /= k as u64;
    }

    if result.is_empty() {
        result.push('0');
    } else {
        result.reverse();
    }

    result
}
