use crate::gargen::vegetables::Asparagus;

pub mod dump;
pub mod gargen;
pub mod generics;
pub mod rect;

//用户
struct User {
    active: bool,
    name: String,
    email: String,
}

#[derive(Debug)] //后面能友好打印
#[allow(dead_code)] // 允许dead_code
struct TrafficLight {
    color: String,
}

// 派生 Debug trait，可以在下面打印这种结构
#[derive(Debug)]
struct Color(i32, i32, i32);

fn main() {
    println!("Hello, world!");

    let a: u8 = 255;

    let b = a.wrapping_add(19);

    println!("{}", b); // 19

    let penguin_data = "\
   common name,length (cm)
   Little penguin,33
   Yellow-eyed penguin,65
   Fiordland penguin,60
   Invalid,data
   ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
        // 只在debug模式下生效，当--release时就不会打印这里
        if cfg!(debug_assertions) {
            // 输出到标准错误输出
            eprintln!("debug : {:?} -> {:?}", record, fields);
        }

        let name = fields[0];

        if let Ok(length) = fields[1].parse::<f32>() {
            // 输出到标准输出
            println!("{}, {}cm", name, length);
        }
    }

    // 导入结构体
    let plant = Asparagus {
        // hello: String::from("哈哈哈"),
        name: String::from("王五"),
    };
    println!("导入的 {:?}!", plant.name);

    // 外部模块
    dump::main();

    // let _s = generics::SingleGen(1);

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

    let rect1 = rect::Rectangle {
        width: 40,
        height: 40,
    };

    println!(
        "矩形宽度:{},矩形高度:{},矩形面积:{},矩形数据是否有效:{}",
        rect1.get_width(),
        rect1.get_height(),
        rect1.area(),
        rect1.valid()
    );
    println!("矩形面积：{}", rect::calc_area(&rect1))
}
