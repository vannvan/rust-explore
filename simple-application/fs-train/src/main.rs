use regex::Regex;
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Result};
use std::path;
use walkdir::{DirEntry, WalkDir};

fn main() {
    println!("文件操作");

    // 创建一个目录，返回 `io::Result<()>`
    match fs::create_dir("a") {
        Err(why) => println!("create_dir 报错! {:?}", why.kind()),
        Ok(_) => {}
    }

    println!("`mkdir -p a/c/d`");
    // 递归地创建一个目录，返回 `io::Result<()>`
    fs::create_dir_all("a/c/d").unwrap_or_else(|why| {
        println!("create_dir_all 报错! {:?}", why.kind());
    });

    println!("`touch a/c/e.txt`");
    touch_2(&path::Path::new("a/c/e.txt")).unwrap_or_else(|why| {
        println!("touch报错 ! {:?}", why.kind());
    });

    println!("`ls a`");
    // 读取目录的内容，返回 `io::Result<Vec<Path>>`
    match fs::read_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                println!("> {:?}", path.unwrap().path());
            }
        }
    }

    let not_exit_file = "不存在的文件.txt";

    let exit_file = "存在的文件.txt";
    if path::Path::new(&not_exit_file).exists() {
        println!("文件存在")
    } else {
        println!("{} 文件不存在", not_exit_file)
    }

    if path::Path::new(&exit_file).exists() {
        println!("{} 文件存在", exit_file);
        match fs::File::open(exit_file) {
            Ok(mut f) => {
                let mut data = String::new();
                f.read_to_string(&mut data).expect("文件读取失败");
                println!("{}", data)
            }
            Err(err) => {
                eprintln!("{} 文件读取失败", err)
            }
        }
    } else {
        println!("文件不存在")
    }

    let file_name = "test.txt";
    let touch_res = touch(
        file_name.to_string(),
        String::from("test文件的内容啊啊哈啊"),
    );

    println!("新建文件的结果,{:?}", touch_res);

    let cp_res = cp(file_name.to_string(), "test-2.txt".to_string());

    println!("拷贝文件的结果,{:?}", cp_res);

    let rename_res = rename("test-2.txt".to_string(), "test-3.txt".to_string());

    println!("修改名称的结果,{:?}", rename_res);

    let remove_res = delete("test-3.txt".to_string());

    println!("删除文件的结果,{:?}", remove_res);

    let _touch_res = touch(
        "test-4.txt".to_string(),
        String::from("test2文件的内容啊啊哈啊"),
    );

    // 遍历目录
    visit_dirs();

    // walkdir方法使用
    walkdir()
}

// `% touch path` 的简单实现（忽略已存在的文件）
fn touch_2(path: &path::Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

// 新建文件
fn touch(file_name: String, content: String) -> Result<u8> {
    fs::write(file_name, content)?;
    Ok(1)
}

#[allow(dead_code)]
fn cp(source_file_name: String, new_file_name: String) -> Result<u8> {
    fs::copy(source_file_name, new_file_name)?;
    Ok(1)
}
#[allow(dead_code)]

fn delete(file_name: String) -> Result<u8> {
    fs::remove_file(file_name)?;
    Ok(1)
}
#[allow(dead_code)]
fn rename(old_name: String, new_name: String) -> Result<u8> {
    fs::rename(old_name, new_name)?;
    Ok(1)
}
#[allow(dead_code)]
// // 遍历文件
// fn map() -> Result<()> {
//     let mut entries = fs::read_dir(".")?
//         .map(|res| res.map(|e| e.path()))
//         .collect::<Result<Vec<_>, Error>>()?;

//     println!("{:?}", entries);

//     Ok(())
// }

// 一种仅通过访问文件来遍历目录的可能实现方式
#[warn(unused_parens)]
fn visit_dirs() {
    match fs::read_dir(".") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                match path {
                    Ok(file) => {
                        if file.file_name().to_str().unwrap().ends_with("txt") {
                            println!("visit_dirs方法 匹配到的txt文件 > {:?}", file.file_name());
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}

fn is_show(entry: &DirEntry) -> bool {
    Regex::new("txt")
        .unwrap()
        .is_match(entry.file_name().to_str().unwrap())
}

fn walkdir() {
    let walker = WalkDir::new(".").into_iter();
    for entry in walker.into_iter() {
        let entry = entry.unwrap();
        if is_show(&entry) {
            println!("walkdir方法 匹配的文件 {}", entry.path().display());
        }
    }
}
