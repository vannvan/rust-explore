fn main() {
    println!("Hello, world!");

    // eg1
    let mut number = 1;
    while number != 4 {
        println!("{}", number);
        number += 1;
    }

    // eg2
    let mut i = 0;
    while i < 5 {
        println!("{}", i);
        i += 1;
    }

    // eg3
    let arr = [1, 2, 3, 4, 5];

    for i in arr.iter() {
        println!("值: {}", i);
    }

    // eg4
    let arr1 = [1, 2, 3, 4, 5, 6];
    for i in 0..5 {
        println!("a[{}] = {}", i, arr1[i]);
    }

    // eg5
    let s = ['H', 'E', 'L', 'L', '0'];

    let mut i = 0;

    let location = loop {
        let ch = s[i];
        if ch == 'E' {
            break i;
        }
        i += 1;
    };

    println!("\'E\'' 的索引值为: {}", location)
}
