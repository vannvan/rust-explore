fn main() {
    let s: String = String::from("哈哈哈啊啊哈哈哈哈");
    another_function(5, s);

    let s = String::from("hello world");

    let word = first_word(&s);

    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    &s[..1]
}

fn another_function(x: i32, y: String) {
    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);
}
