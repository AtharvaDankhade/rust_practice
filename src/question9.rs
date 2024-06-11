fn reverse_string(input: &str) -> String {
    input.chars().rev().collect::<String>()
}

fn main() {
    let original_string = "Hello, world!";
    let reversed_string = reverse_string(original_string);
    println!("Original string: {}", original_string);
    println!("Reversed string: {}", reversed_string);
}
