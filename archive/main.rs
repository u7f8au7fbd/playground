#[derive(Debug, serde::Serialize)]
struct QueryData {
    main: String,
    sub: Vec<String>,
}

#[macro_use]
mod macros;

use serde_json::Value;
use std::{thread, time};

fn setup() {
    cmd!(clear); // clearコマンドを実行する
    cmd!(utf8); // utf-8コマンドを実行する
    cmd!(red_line); // lineコマンドを実行する
}

fn read_and_print_json(path: &str) -> Result<Vec<QueryData>, Box<dyn std::error::Error>> {
    let mut data_list: Vec<QueryData> = Vec::new();

    let json_str = std::fs::read_to_string(path)?;

    let json: Value = serde_json::from_str(&json_str)?;

    if let Value::Array(arr) = json {
        for obj in arr {
            if let Value::String(main_word) = obj["main_word"].clone() {
                if let Value::Array(sub_word) = obj["sub_word"].clone() {
                    let mut sub: Vec<String> = Vec::new();

                    for word in sub_word {
                        if let Value::String(word) = word {
                            let url = format!("{}+{}", main_word, word);
                            sub.push(url);
                        }
                    }
                    let data_entry = QueryData {
                        main: main_word,
                        sub,
                    };
                    data_list.push(data_entry);
                }
            }
        }
    }
    Ok(data_list)
}

fn get_now_time() -> String {
    let now = chrono::Local::now();
    now.format("%Y-%m-%d_%H-%M-%S").to_string()
}

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

use std::sync::Arc;
use tokio::task;

async fn process_in_chunks(num_str: Vec<String>, path: Arc<str>, chunk_size: usize) {
    let num_str_with_index: Vec<(usize, String)> = num_str.into_iter().enumerate().collect();
    for chunk in num_str_with_index.chunks(chunk_size) {
        let mut handles = Vec::new();

        for (index, num) in chunk {
            let num = num.clone(); // この部分のcloneはやむを得ない
            let index = *index; // インデックスをコピー
            let path = Arc::clone(&path);

            let handle = task::spawn(async move {
                download_html(&num, &format!("{}/{}.html", &path, index))
                    .await
                    .unwrap();
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }
    }
}

async fn get_query(word: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut attempt = 0;
    let max_attempts = 10;

    while attempt < max_attempts {
        attempt += 1;
        let url:&str = &format!("https://www.google.co.jp/search?q={}&oq={}&hl=ja&lr=lang_ja&pws=0&sourceid=chrome&ie=UTF-8&num=100", word, word);
        let mut urls: Vec<String> = Vec::new();

        // ランダムなユーザーエージェントを選択
        let user_agent =
            USER_AGENTS_INDEX[rand::thread_rng().gen_range(0..USER_AGENTS_INDEX.len())];
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

        println!("Attempt {}: URLs count: {}", attempt, urls.len());

        if !urls.is_empty() {
            return Ok(urls);
        }

        // 再試行する前に少し待つ
        let wait_time = rand::thread_rng().gen_range(40..80);
        println!("Wait:{}", wait_time);
        tokio::time::sleep(time::Duration::from_secs(wait_time)).await;
    }

    Err("データの取得に失敗".into())
}

async fn download_html(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // ユーザーエージェントの配列からランダムに1つを選ぶ
    let user_agent = USER_AGENTS_INDEX[rand::thread_rng().gen_range(0..USER_AGENTS_INDEX.len())];
    let mut headers = HeaderMap::new();

    // ヘッダーにユーザーエージェントを追加
    headers.insert(USER_AGENT, HeaderValue::from_str(user_agent)?);
    // ヘッダーに空のクッキーを追加
    headers.insert("Cookie", HeaderValue::from_str("")?);

    // クッキーをサポートするクライアントをビルド
    let client = Client::builder().cookie_store(true).build()?;

    // エラーハンドリングのためにマッチ式を使用
    match client.get(url).headers(headers).send().await {
        Ok(response) => {
            // 指定されたファイルパスにファイルを作成または開く
            let mut file = OpenOptions::new()
                .create(true) // ファイルが存在しない場合は作成
                .write(true) // 書き込みモードで開く
                .truncate(true) // ファイルの内容を消去
                .open(file_path)
                .await?;

            // レスポンスの内容をテキストとして取得
            let content = response.text().await?;
            // ファイルに書き込む
            file.write_all(content.as_bytes()).await?;
        }
        Err(e) => {
            // エラーが発生した場合はエラーメッセージとURLを表示
            println!("Failed to download {}: {}", url, e);
        }
    }

    Ok(())
}
//////////////////////////////////////////////////////////////////////////////////////////////////
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    let path = "./sample.json";
    let data_list = read_and_print_json(path)?;
    println!("{:#?}", data_list);

    // Create a directory
    let dir = format!("./db/{}", get_now_time());
    println!("{}: Finished", get_now_time());
    std::fs::create_dir_all(&dir)?;

    for data in &data_list {
        println!("{}", data.main);
        std::fs::create_dir_all(format!("{}/{}", &dir, data.main))?;
        for sub in &data.sub {
            let wait_time = rand::thread_rng().gen_range(40..80);
            println!("Wait:{}", wait_time);
            tokio::time::sleep(time::Duration::from_secs(wait_time)).await;

            println!("{}", sub);
            std::fs::create_dir_all(format!("{}/{}/{}", &dir, data.main, sub))?;
            let word = get_query(sub).await?;
            let chunk_size = 100;
            // `Vec<String>`を渡して所有権の問題を回避します。
            process_in_chunks(
                word,
                format!("{}/{}/{}", &dir, data.main, sub).into(),
                chunk_size,
            )
            .await;
        }
    }

    // Serialize data_list to JSON
    let json_str = serde_json::to_string_pretty(&data_list)?;
    // Write JSON to output file
    std::fs::write(format!("{}/log.json", &dir), json_str)?;
    Ok(())
}
