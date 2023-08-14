// 矩形
pub(crate) struct Rectangle {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

// 可以理解为一个类实现
impl Rectangle {
    pub(crate) fn area(&self) -> u32 {
        self.width * self.height
    }

    pub(crate) fn get_height(&self) -> u32 {
        self.height
    }
    pub(crate) fn get_width(&self) -> u32 {
        self.width
    }

    pub(crate) fn valid(&self) -> bool {
        self.width > 0 && self.height > 0
    }
}

// 计算矩形面积的方法
pub(crate) fn calc_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
