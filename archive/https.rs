use rand::Rng;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Client;
use scraper::{Html, Selector};
use std::{thread, time};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

const USER_AGENTS_INDEX: [&str; 10] = [
    // Windows用のChromeブラウザ
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
    // Windows用のFirefoxブラウザ
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:89.0) Gecko/20100101 Firefox/89.0",
    // Windows用のEdgeブラウザ
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36 Edg/91.0.864.59",
    // Macintosh用のSafariブラウザ
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0.3 Safari/605.1.15",
    // Macintosh用のChromeブラウザ
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
    // Ubuntu用のFirefoxブラウザ
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:89.0) Gecko/20100101 Firefox/89.0",
    // Ubuntu用のChromeブラウザ
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
    // iPhone用のSafariブラウザ
    "Mozilla/5.0 (iPhone; CPU iPhone OS 14_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1",
    // iPhone用のChromeブラウザ
    "Mozilla/5.0 (iPhone; CPU iPhone OS 14_6 like Mac OS X) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Mobile Safari/537.36",
    // Android用のChromeブラウザ
    "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Mobile Safari/537.36"
];

async fn download_html(word: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url:&str = &format!("https://www.google.com/search?q={}&oq={}&hl=ja&lr=lang_ja&pws=0&sourceid=chrome&ie=UTF-8&num=10&start=0", word,word);
    let user_agent = USER_AGENTS_INDEX[rand::thread_rng().gen_range(0..USER_AGENTS_INDEX.len())];
    let mut headers = HeaderMap::new();

    headers.insert(USER_AGENT, HeaderValue::from_str(user_agent)?);
    headers.insert("Cookie", HeaderValue::from_str("")?);

    let client = Client::builder().cookie_store(true).build()?;
    let response = client.get(url).headers(headers).send().await?;

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)
        .await?;

    let content = response.text().await?;
    file.write_all(content.as_bytes()).await?;
    Ok(())
}

async fn get_html_status(word: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url:&str = &format!("https://www.google.com/search?q={}&oq={}&hl=ja&lr=lang_ja&pws=0&sourceid=chrome&ie=UTF-8&num=10&start=0", word,word);
    let user_agent = USER_AGENTS_INDEX[rand::thread_rng().gen_range(0..USER_AGENTS_INDEX.len())];
    let mut headers = HeaderMap::new();

    headers.insert(USER_AGENT, HeaderValue::from_str(user_agent)?);
    headers.insert("Cookie", HeaderValue::from_str("")?);

    let client = Client::builder().cookie_store(true).build()?;
    let response = client.get(url).headers(headers).send().await?;

    println!("HTML status: {}", response.status());
    Ok(())
}

async fn get_html_content(word: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url:&str = &format!("https://www.google.com/search?q={}&oq={}&hl=ja&lr=lang_ja&pws=0&sourceid=chrome&ie=UTF-8&num=10&start=0", word,word);
    let user_agent = USER_AGENTS_INDEX[rand::thread_rng().gen_range(0..USER_AGENTS_INDEX.len())];
    let mut headers = HeaderMap::new();

    headers.insert(USER_AGENT, HeaderValue::from_str(user_agent)?);
    headers.insert("Cookie", HeaderValue::from_str("")?);

    let client = Client::builder().cookie_store(true).build()?;
    let response = client.get(url).headers(headers).send().await?;
    let content = response.text().await?;
    println!("HTML content:\n{}", content);
    Ok(())
}

async fn get_query(word: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url:&str = &format!("https://www.google.com/search?q={}&oq={}&hl=ja&lr=lang_ja&pws=0&sourceid=chrome&ie=UTF-8&num=10&start=0", word,word);
    let mut urls: Vec<String> = Vec::new();
    // ランダムなユーザーエージェントを選択
    let user_agent = USER_AGENTS_INDEX[rand::thread_rng().gen_range(0..USER_AGENTS_INDEX.len())];
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_str(user_agent)?);
    headers.insert("Cookie", HeaderValue::from_str("")?);

    // HTTPクライアントの作成
    let client = Client::builder().cookie_store(true).build()?;
    let response = client.get(url).headers(headers).send().await?;
    let content = response.text().await?;

    // HTMLパース
    let document = Html::parse_document(&content);
    let selector = Selector::parse(r#"a[jsname="UWckNb"]"#).unwrap();

    // href属性の抽出と出力
    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            urls.push(href.to_string());
        }
    }

    println!("URLs: {:#?}", urls);
    println!("URLs count: {}", urls.len());
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "./test/test.html";
    get_query("転職").await?;
    Ok(())
}
