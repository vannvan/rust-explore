#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
}

fn main() {
    println!("集合类型!");
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);

    let first = &v[0];
    let second = &v.get(1);

    println!("拿到的值: {first} {:?}", second);

    for i in &v {
        println!("{i}");
    }

    let ip_arr = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];

    println!("ip_arr: {:?}", ip_arr);

    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort_unstable();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);

    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("Al".to_string(), 30),
        Person::new("John".to_string(), 1),
        Person::new("John".to_string(), 25),
    ];

    people.sort_unstable();
    println!("{:?}", people);
}
