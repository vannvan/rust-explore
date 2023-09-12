use std::io::{BufRead, BufReader};
use std::process::{self, Command, Stdio};
fn main() {
    let pull = Command::new("git")
        .arg("pull")
        .output()
        .expect("pull执行异常，提示");
    println!("{}", String::from_utf8(pull.stdout).unwrap());

    let add = Command::new("git")
        .args(&["add", "."])
        .output()
        .expect("add执行异常，提示");

    println!("{}", String::from_utf8(add.stdout).unwrap());

    let status = Command::new("git")
        .arg("status")
        .output()
        .expect("status执行异常，提示");
    println!("{}", String::from_utf8(status.stdout).unwrap());

    let commit = Command::new("git")
        .args(&["commit", "-m", "update"])
        .output()
        .expect("commit执行异常，提示");
    println!("{}", String::from_utf8(commit.stdout).unwrap());
}
