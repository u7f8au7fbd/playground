use rand::Rng;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::{thread, time};
use tokio::task;

#[macro_use]
mod macros;

fn get_https_status(url: &str) -> Result<reqwest::StatusCode, Box<dyn std::error::Error>> {
    // ユーザーエージェントのリスト
    let user_agents = [
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

    // ランダムにユーザーエージェントを選択
    let mut rng = rand::thread_rng();
    let user_agent = user_agents[rng.gen_range(0, user_agents.len())];

    // HTTPクライアントの作成
    let client = Client::builder()
        .cookie_store(true) // クッキーを有効にする
        .build()?;

    // HTTPリクエストのヘッダーを設定
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_str(user_agent)?);

    // ランダムな待機時間（1～5秒）を設定してリクエストを送信
    let wait_time = time::Duration::from_secs(rng.gen_range(2, 3));
    thread::sleep(time::Duration::from_millis(2000));

    let response = client.get(url).headers(headers).send()?;

    // ステータスコードを返す
    Ok(response.status())
}

fn main() {
    // タスクを実行
    running_task();
}

fn running_task() {
    time_count!({
        for i in 0..65535 {
            print!("{}:", i + 1);
            let url = format!("https://www.google.co.jp/search?q={}&start=0", i);

            match get_https_status(&url) {
                Ok(status) => {
                    if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                        eprintln!("{}{}{}", cmd_color!(red), status, cmd_color!(reset));
                        std::process::exit(0);
                    } else {
                        println!("{}{}{}", cmd_color!(green), status, cmd_color!(reset));
                    }
                }
                Err(e) => eprintln!("{}{}{}", cmd_color!(red), e, cmd_color!(reset)),
            }
        }
    })
}
