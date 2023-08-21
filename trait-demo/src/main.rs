// 一个Draw特征
pub trait Draw {
    fn draw(&self);
}

pub struct Buttton {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

// 为按钮实现Draw特征
impl Draw for Buttton {
    fn draw(&self) {
        // 绘制按钮的代码
        println!(
            "按钮信息-> width:{}, height:{}, label:{}",
            self.width, self.height, self.label
        )
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

// 为SelectBox实现Draw特征
impl Draw for SelectBox {
    fn draw(&self) {
        // 绘制SelectBox的代码
        println!(
            "SelectBox信息-> width: {}, height: {}, options: {:?}",
            self.width, self.height, self.options
        )
    }
}
//
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn render(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main() {
    println!("绘制组件列表");

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 100,
                height: 32,
                options: vec![String::from('a'), String::from('b'), String::from('c')],
            }),
            Box::new(Buttton {
                width: 60,
                height: 40,
                label: String::from("确认"),
            }),
        ],
    };

    screen.render();
}
