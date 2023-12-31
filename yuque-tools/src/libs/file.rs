/*
 * Description: 文件操作
 * Created: 2023-08-31 22:09:40
 * Author: vannvan
 * Email : adoerww@gmail.com
 * -----
 * Copyright (c) https://github.com/vannvan
 */

use std::{
    fs::{self, OpenOptions},
    io::{Error, Read, Write},
    path,
};
#[allow(unused_imports)]

/// 文件
pub struct File {}
#[allow(dead_code)]
impl File {
    pub fn new() -> Self {
        File {}
    }

    /// 判断是否存在
    pub fn exists(&self, f: &str) -> bool {
        if path::Path::new(f).exists() {
            return true;
        }

        false
    }

    /// 删除文件
    pub fn remove(&self, f: &str) -> Result<(), Error> {
        fs::remove_file(f)?;

        Ok(())
    }

    /// 创建文件
    pub fn create(&self, f: &str) -> Result<fs::File, Error> {
        let file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(f)?;

        Ok(file)
    }

    /// 读取
    pub fn read(&self, f: &str) -> Result<String, Error> {
        let mut file = fs::File::open(f)?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let contents = String::from_utf8_lossy(&buffer).to_string();

        Ok(contents)
    }

    /// 写入信息
    pub fn write(&self, f: &str, content: String) -> Result<(), Error> {
        let path = path::Path::new(f);

        let mut file = fs::File::create(path)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }

    /// 追加内容
    pub fn append(&self, f: &str, content: String) -> Result<(), Error> {
        let path = path::Path::new(f);

        let mut file = OpenOptions::new()
            .append(true)
            .open(path)
            .expect("cannot open file");
        file.write_all(content.as_bytes()).expect("write failed");

        Ok(())
    }

    /// 创建文件夹
    pub fn mkdir(&self, d: &str) -> Result<(), Error> {
        fs::create_dir_all(d)?;

        Ok(())
    }

    /// 删除文件夹
    pub fn rmdir(&self, d: &str) -> Result<(), Error> {
        fs::remove_dir_all(d)?;

        Ok(())
    }
}

#[test]
fn test() {
    let f = File::new();

    let cookies = "啊哈哈啊".to_string();
    let dir = ".meta".to_string();
    let file = dir + "/test.json";
    match f.mkdir(".meta") {
        Ok(_) => match f.write(&file, cookies) {
            Ok(_) => {
                println!("创建成功")
            }
            _ => {
                println!("文件创建失败");
            }
        },
        Err(_err) => {
            println!("元目录创建失败");
        }
    }
}

#[test]
fn touch_file() {
    let f = File::new();
    let dir = "./dev/tmp/".to_string();

    for i in 0..10 {
        let file = format!("{}/三级{}.txt", dir, i);
        let content = format!("测试内容,{}", i);

        let _ = f.write(&file, content);
    }
}

#[test]
fn touch_file_and_mkdir() {
    let path = std::path::Path::new("./home/roger/foo/bar/baz.txt");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
}
