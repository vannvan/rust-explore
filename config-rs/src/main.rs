use config::{Config, File};
// use glob::glob;
use std::collections::HashMap;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    let key = "title";
    let value = get_config(key);

    println!("{}", value)
}

fn get_config(key: &str) -> String {
    let settings = Config::builder()
        // .add_source(File::with_name("examples/glob/conf/00-default.toml"))
        // .add_source(File::from(Path::new("examples/glob/conf/05-some.yml")))
        .add_source(File::from(Path::new("config.json")))
        .build()
        .unwrap();

    let config = settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();

    let ss = config.get(key);
    // println!("\n{:?} \n\n-----------", config);
    // println!("{:}", ss.unwrap().to_string());
    ss.unwrap().to_string()
}
