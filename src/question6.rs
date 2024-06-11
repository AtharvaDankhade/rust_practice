fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let first_string = strings[0];
    let mut prefix = String::new();

    'outer: for (i, char) in first_string.chars().enumerate() {
        for string in &strings[1..] {
            if let Some(c) = string.chars().nth(i) {
                if c != char {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
        prefix.push(char);
    }

    prefix
}

fn main() {
    let strings1 = ["flower", "flow", "flight"];
    let strings2 = ["dog", "racecar", "car"];

    println!("Longest common prefix of strings1: {}", longest_common_prefix(&strings1));
    println!("Longest common prefix of strings2: {}", longest_common_prefix(&strings2)); 
}
