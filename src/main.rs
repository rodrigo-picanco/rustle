use yansi::Paint;

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

fn read() -> String { 
    let mut result = String::new();
    std::io::stdin().read_line(&mut result).unwrap();
    result.trim().to_string()
}

fn inform(result: Vec<u8>, guess: String) {
    for (i, c) in guess.chars().enumerate() {
        match result[i] {
            2 => print!("{}", Paint::green(c)),
            1 => print!("{}", Paint::yellow(c)),
            _ => print!("{}", Paint::red(c)),
        }
    };
}

fn run() {
    let word = "hello";
    let guess = read();
    let result = compare(word, &guess);
    inform(result, guess);
}

fn main() {
    println!("What's the word?");
    run()
}

#[test]
fn it_can_compare_words() {
    assert_eq!(compare("hello", "hello"), [2, 2, 2, 2, 2]);
    assert_eq!(compare("hello", "halle"), [2, 0, 2, 2, 1]);
}
