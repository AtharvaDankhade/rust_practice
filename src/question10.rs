fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() {
    let test_cases = [1, 2, 3, 4, 5, 16, 17, 18, 19, 20, 23, 24, 29];

    for &num in &test_cases {
        println!("Is {} prime? {}", num, is_prime(num));
    }
}
