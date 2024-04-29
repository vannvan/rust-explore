use webbrowser;

fn main() {
    if webbrowser::open("http://github.com").is_ok() {
        // ...
        println!("Hello, world!");
    }
}
