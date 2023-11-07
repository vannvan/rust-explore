use reqwest;
use scraper::{Html, Selector};
use thirtyfour::{error::WebDriverError, prelude::*};
use tokio;

#[tokio::main]
#[allow(dead_code)]
async fn main() {
    // let _ = get_html().await;

    let s = webdriver_handler().await;

    println!("{:#?}", s)
}

async fn webdriver_handler() -> Result<(), WebDriverError> {
    let url = "https://spa1.scrape.center/";
    let caps = DesiredCapabilities::chrome();

    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.goto(url).await?;
    // 等待我们要的元素
    let check = driver.query(By::Css(".m-b-sm")).first().await?;
    check.wait_until().displayed().await?;
    let els = driver.find_all(By::Css(".m-b-sm")).await?;
    for el in &els {
        println!("el:{}", el.inner_html().await?.as_str());
    }

    // println!("{:#?}", els);
    driver.quit().await?;
    Ok(())
}

async fn get_html() -> Result<(), reqwest::Error> {
    // HTML
    let url = "https://ssr1.scrape.center/";
    let resp = reqwest::get(url).await?;
    //println!("Body:{:#?}",resp.text().await?);
    let body = resp.text().await?;
    let doc = Html::parse_fragment(&body);
    let selector = Selector::parse(".m-b-sm").unwrap();
    for el in doc.select(&selector) {
        println!("title:{}", el.inner_html());
    }
    // JSON
    let url = "https://spa1.scrape.center/api/movie/?limit=10&offset=0";
    let resp = reqwest::get(url).await?;
    let json_body: serde_json::Value = resp.json().await?;
    //println!("Json:{:#?}",json_body);
    let json_sel = jsonpath::Selector::new("$.results.*.name").unwrap();
    for json_el in json_sel.find(&json_body) {
        println!("json title:{}", json_el.as_str().unwrap());
    }

    Ok(())
}

async fn get_window() {
    let response = reqwest::get("https://meitulu.me/item/4756.html")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let document = scraper::Html::parse_document(&response);
}
