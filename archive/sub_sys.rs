#[macro_use]
mod macros;

use rand::Rng;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Client;
use scraper::{Html, Selector};
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

fn setup() {
    cmd!(clear); // clearコマンドを実行する
    cmd!(utf8); // utf-8コマンドを実行する
    cmd!(red_line); // lineコマンドを実行する
}

use tokio::task;
async fn process_in_chunks(num_str: Vec<String>, chunk_size: usize) {
    let num_str_with_index: Vec<(usize, String)> = num_str.into_iter().enumerate().collect();
    let mut counter = 0;

    for chunk in num_str_with_index.chunks(chunk_size) {
        let mut handles = Vec::new();

        for (index, num) in chunk {
            let num = num.clone(); // この部分のcloneはやむを得ない
            let index = *index; // インデックスをコピー

            let handle = task::spawn(async move {
                download_html(&num, &format!("./test/{}.html", index))
                    .await
                    .unwrap();
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }
        counter += 1;
        print!("{}/{}", counter, num_str_with_index.len() / chunk_size);
        cmd!(green_line); //1チャンクの処理が終わるたびに緑の線を表示する
    }
}

async fn get_query(word: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let url:&str = &format!("https://www.google.com/search?q={}&oq={}&hl=ja&lr=lang_ja&pws=0&sourceid=chrome&ie=UTF-8&num=100&start=0", word,word);
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
    Ok(urls)
}

async fn download_html(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    let word = get_query("Rust").await?;
    time_count!({
        let chunk_size = 100;
        // `Vec<String>`を渡して所有権の問題を回避します。
        process_in_chunks(word, chunk_size).await;
    });
    Ok(())
}
