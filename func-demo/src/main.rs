fn main() {
    let s: String = String::from("哈哈哈啊啊哈哈哈哈");
    another_function(5, s);

    let s = String::from("hello world");

    let word = first_word(&s);

    println!("the first word is: {}", word);

    // 闭包1
    fn function(i: u32) -> u32 {
        i + 1
    }

    // 闭包2
    let closure_annotated = |i: i32| -> i32 { i + 1 };

    // 闭包3
    let closure_inferred = |i| i + 1;

    println!("{}", function(1));
    println!("{}", closure_annotated(2));
    println!("{}", closure_inferred(3));

    let color = String::from("red");

    let print = || println!("color: {}", color);

    // 使用借用来调用闭包 color
    print();

    let _borrow = &color;

    print();

    let _color_remove = color;

    let hello = String::from("hello");

    let diary = || println!("say {}", hello);

    // 以闭包作为参数，调用函数 `apply`
    apply(diary);

    // 闭包 `double` 满足 `apply_to_3` 的 trait 约束。
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));

    // 定义一个满足 `Fn` 约束的闭包。
    let closure = || println!("我是一个闭包函数!");

    call_me(closure);
    call_me(function_1);
}

// 定义一个满足 `Fn` 约束的封装函数（wrapper function）。
fn function_1() {
    println!("我是一个普通函数!");
}

// 定义一个函数，可以接受一个由 `Fn` 限定的泛型 `F` 参数并调用它。
fn call_me<F: Fn()>(f: F) {
    f()
}

// 该函数将闭包作为参数并调用它。
fn apply<F>(f: F)
where
    // 闭包没有输入值和返回值。
    F: FnOnce(),
{
    // ^ 试一试：将 `FnOnce` 换成 `Fn` 或 `FnMut`。

    f();
}

// 输入闭包，返回一个 `i32` 整型的函数。
fn apply_to_3<F>(f: F) -> i32
where
    // 闭包处理一个 `i32` 整型并返回一个 `i32` 整型。
    F: Fn(i32) -> i32,
{
    f(3)
}

fn first_word(s: &String) -> &str {
    &s[..1]
}

fn another_function(x: i32, y: String) {
    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);
}
