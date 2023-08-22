// 模式匹配
enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}
pub(crate) fn main() {
    let actions = [
        Action::Say("hello world".to_string()),
        Action::MoveTo(1, 0),
        Action::ChangeColorRGB(255, 255, 0),
    ];

    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s)
            }
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            }
            Action::ChangeColorRGB(r, g, _) => {
                println!(
                    "change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }
    multi()
}

pub(crate) fn part_match() {
    let actions = [
        Action::Say("hello world".to_string()),
        Action::MoveTo(1, 0),
        Action::ChangeColorRGB(255, 255, 0),
    ];

    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s)
            }
            _ => (),
        }
    }
}

pub(crate) fn if_let() {
    let v = Action::Say("hello".to_string());
    if let Action::Say(_s) = &v {
        println!("if let 是Say方法执行力了")
    }

    // 这里就执行不到
    if let Action::MoveTo(_x, _y) = &v {
        println!("if let 是MoveTo方法执行力了")
    }
}

pub(crate) fn multi() {
    // eg1 通过序列 ..= 匹配值的范围
    let x = 5;
    match x {
        1..=6 => {
            println!("在1-6范围内")
        }
        _ => {
            println!("在其它范围内")
        }
    }

    // eg2
    let five = Some(5);
    let six = plus_one(five);
    println!("plus_one的结果{:?}", six); // plus_one的结果Some(6)

    // eg3
    // Vec是动态数组
    let mut stack = Vec::new();

    // 向数组尾部插入元素
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // stack.pop从数组尾部弹出元素
    while let Some(top) = stack.pop() {
        println!("Vec数组元素 {}", top);
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
