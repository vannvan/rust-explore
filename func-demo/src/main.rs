fn main() {
    let s: String = String::from("哈哈哈啊啊哈哈哈哈");
    another_function(5, s);
}

fn another_function(x: i32, y: String) {
    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);
}
