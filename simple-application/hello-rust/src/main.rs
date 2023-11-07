use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    println!("Hello, world!");

    let stdout = stdout();
    let message = String::from("哈哈哈啊啊哈哈哈哈");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap()
}
