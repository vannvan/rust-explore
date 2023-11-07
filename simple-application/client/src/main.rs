use serde::Deserialize;
use serde_json::Value;
// #![deny(warnings)]

#[derive(Deserialize, Debug)]
#[allow(dead_code)] // 允许dead_code
struct Response<T> {
    success: bool,
    data: T,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // // 从入参接收
    // let url = if let Some(url) = std::env::args().nth(1) {
    //     url
    // } else {
    //     let default = String::from("https://api.vvhan.com/api/horoscope?type=scorpio&time=today");
    //     println!("没有传入URL，使用默认链接了.");
    //     default.into()
    // };

    let url = String::from("https://api.vvhan.com/api/horoscope?type=scorpio&time=today");

    eprintln!("Fetching {:?}...", url);

    // let res = reqwest::get(url).await?.json::<Response>().await?;

    // let _char: Response<char> = Response('a');
    #[derive(Deserialize, Debug)]
    struct Data {
        title: String,
        time: String,
    }

    let res = reqwest::get(url).await?.json::<Response<Data>>().await?;

    // eprintln!("Response: {:?} {}", res.version(), res.status());
    // eprintln!("Headers: {:#?}\n", res.headers());

    // let body = res.text().await?;

    println!("{},{},{}", res.success, res.data.time, res.data.title);

    println!("{:?}", res);

    json();

    Ok(())
}

fn json() {
    let json = r#"
{
  "article": "哈哈哈 xxx",
  "author": "作者 xxx",
  "paragraph": [
    {
      "name": "untyped"
    },
    {
      "name": "strongly typed"
    },
    {
      "name": "writing json"
    }
  ]
}
"#;

    let parsed: Value = read_json(json);
    println!("\n\n The title of the article is {}", parsed["article"])
}

fn read_json(raw_json: &str) -> Value {
    let parsed: Value = serde_json::from_str(raw_json).unwrap();
    return parsed;
}
