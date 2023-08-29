use owo_colors::OwoColorize;

#[allow(dead_code)]
pub enum Action {
    SUCCESS,
    INFO,
    ERROR,
    WARN,
}
#[allow(dead_code)]
pub fn dump_log(action: Action, str: String) {
    const NAME: &str = "ytool->";
    match action {
        Action::SUCCESS => {
            println!("{NAME} {}", str.green())
        }
        Action::INFO => {
            println!("{NAME} {}", str.cyan())
        }
        Action::ERROR => {
            println!("{NAME} {}", str.red())
        }
        Action::WARN => {
            println!("{NAME} {}", str.yellow())
        }
    }
}

pub struct Log;

const NAME: &str = "ytool->";
#[allow(dead_code)]
impl Log {
    pub fn success(str: &str) {
        println!("{NAME} {}", str.green())
    }

    pub fn info(str: &str) {
        println!("{NAME} {}", str.cyan())
    }
    pub fn error(str: &str) {
        println!("{NAME} {}", str.red())
    }
    pub fn warn(str: &str) {
        println!("{NAME} {}", str.yellow())
    }
}
