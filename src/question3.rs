fn find_shortest_word(s: &str) -> &str {
    let mut shortest_word = "";
    let mut shortest_length = usize::MAX;

    for word in s.split_whitespace() {
        let word_length = word.len();
        if word_length < shortest_length {
            shortest_word = word;
            shortest_length = word_length;
        }
    }

    shortest_word
}

fn main() {
    let sentence = "Rust programming is fun and efficient";
    let shortest_word = find_shortest_word(sentence);
    println!("The shortest word in the sentence is: {}", shortest_word);
}
