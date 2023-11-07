#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct Person<'a> {
    pub(crate) name: &'a str,
    pub(crate) age: u8,
}

pub(crate) fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // 美化打印
    println!("{:#?}", peter);
}
