use crate::gargen::vegetables::Asparagus;

pub mod gargen;

//用户
struct User {
    active: bool,
    name: String,
    email: String,
}

// 派生 Debug trait，可以在下面打印这种结构
#[derive(Debug)]
struct Color(i32, i32, i32);

// 矩形
struct Rectangle {
    width: u32,
    height: u32,
}

// 可以理解为一个类实现
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn get_height(&self) -> u32 {
        self.height
    }
    fn get_width(&self) -> u32 {
        self.width
    }

    fn valid(&self) -> bool {
        self.width > 0 && self.height > 0
    }
}

fn main() {
    println!("Hello, world!");
    let plant = Asparagus {
        // hello: String::from("哈哈哈"),
        name: String::from("王五"),
    };
    println!("导入的 {:?}!", plant.name);

    let num = 3;
    if num > 5 {
        println!("条件成立")
    } else {
        println!("条件不成立")
    }

    let mut user1 = User {
        active: true,
        name: String::from("张三"),
        email: String::from("2672782@qq.com"),
    };

    user1.name = String::from("李四");

    println!("用户信息：{},{},{}", user1.name, user1.email, user1.active);

    let black = Color(0, 0, 0);

    println!("{}喜欢 {:?}", user1.name, black);

    let rect1 = Rectangle {
        width: 40,
        height: 40,
    };

    println!(
        "矩形宽度:{},矩形高度:{},矩形面积:{},矩形信息是否有效:{}",
        rect1.get_width(),
        rect1.get_height(),
        rect1.area(),
        rect1.valid()
    );
    println!("矩形面积：{}", calc_area(&rect1))
}

// 计算矩形面积的方法
fn calc_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
