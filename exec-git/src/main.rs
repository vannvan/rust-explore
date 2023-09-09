fn main() {
    // let err = exec::Command::new("echo").arg("hello").arg("world").exec();

    //  let info = exec::Command::new("git add .").exec();

    let status = exec::Command::new("git").arg("status").exec();

    let pull = exec::Command::new("git").arg("pull").exec();

    let add = exec::Command::new("git").arg("add .").exec();

    // let commit = exec::Command::new("git").arg("commit -m update").exec();

    // let push = exec::Command::new("git").arg("push").exec();

    println!("status: {},pull: {}", status, pull);
}
