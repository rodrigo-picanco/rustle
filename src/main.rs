fn run(mut writer: impl std::io::Write){
    match writeln!(writer, "hello world") {
        Ok(_) => (),
        Err(e) => panic!("failed to write to stdout: {}", e),
    };
}

fn main() {}

#[test]
fn it_runs() {
    let mut result = Vec::new();
    run(&mut result);
    assert_eq!(result, b"hello world\n");
}
