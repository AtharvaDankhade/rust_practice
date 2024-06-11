fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    cleaned == cleaned.chars().rev().collect::<String>()
}

fn main() {
    let test_cases = vec![
        ("A man, a plan, a canal, Panama", true),
        ("racecar", true),
        ("hello", false),
        ("", true),
        ("No 'x' in Nixon", true),
    ];

    for (input, expected) in test_cases {
        let result = is_palindrome(input);
        println!(
            "\"{}\" is a palindrome? {} (Expected: {})",
            input, result, expected
        );
    }
}
