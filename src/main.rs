fn compare(a: &str, b: &str) -> Vec<u8> {
    let mut result = vec![0, 0, 0, 0, 0];

    for (i, c) in b.chars().enumerate() {
        if c == a.chars().nth(i).unwrap() {
            result[i] = 2;
        } else if a.contains(c) {
            result[i] = 1;
        }
    }

    result
}

fn main() {
    println!("What's the word?");

    let mut word = String::new();
    std::io::stdin().read_line(&mut word).unwrap();

    let result = compare("hello", &word.trim());

    println!("Result: {:?}", result);
}

#[test]
fn it_can_compare_words() {
    assert_eq!(compare("hello", "hello"), [2, 2, 2, 2, 2]);
    assert_eq!(compare("hello", "halle"), [2, 0, 2, 2, 1]);
}
