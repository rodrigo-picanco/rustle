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

fn run() {
    let word = "hello";

    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess).unwrap();

    let result = compare(word, &guess.trim());
    for (i, c) in guess.trim().chars().enumerate() {
        match result[i] {
            2 => print!("{}", Paint::green(c)),
            1 => print!("{}", Paint::yellow(c)),
            _ => print!("{}", Paint::red(c)),
        }
    };
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
