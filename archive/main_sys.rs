#[derive(Debug, serde::Serialize)]
struct QueryData {
    main: String,
    sub: Vec<Vec<String>>,
}

use serde_json::Value;
use tokio::task;

#[macro_use]
mod macros;

fn setup() {
    cmd!(clear); // clearコマンドを実行する
    cmd!(utf8); // utf-8コマンドを実行する
    cmd!(red_line); // lineコマンドを実行する
}

fn read_and_print_json(path: &str) -> Result<Vec<QueryData>, Box<dyn std::error::Error>> {
    let mut data_list: Vec<QueryData> = Vec::new(); //最終的に返すデータの配列を定義

    let json_str = std::fs::read_to_string(path)?; //jsonを読み込む

    let json: Value = serde_json::from_str(&json_str)?; //jsonをパースする

    if let Value::Array(arr) = json {
        for obj in arr {
            //配列の中身を取り出す
            if let Value::String(main_word) = obj["main_word"].clone() {
                // main_wordを取り出す
                if let Value::Array(sub_word) = obj["sub_word"].clone() {
                    // sub_wordを取り出す
                    let mut sub: Vec<Vec<String>> = Vec::new(); // Data構造体のdataフィールドに格納するための配列

                    for word in sub_word {
                        // sub_wordの中身を取り出す
                        if let Value::String(word) = word {
                            // sub_wordの中身がString型だった場合
                            let mut page_urls: Vec<String> = Vec::new(); // ページごとのURLを格納するための配列
                            for page in 0..10 {
                                // 1ページから10ページまでのURLを生成
                                let url = format!(
                                    "https://www.google.com/search?q={}+{}&start={}",
                                    main_word,
                                    word,
                                    page * 10
                                ); // URLを生成
                                page_urls.push(url); // ページごとのURLを配列に格納
                            }
                            sub.push(page_urls); // ページごとのURLの配列をdataに格納
                        }
                    }
                    let data_entry = QueryData {
                        main: main_word,
                        sub,
                    };
                    data_list.push(data_entry); // Data構造体をdata_listに格納
                }
            }
        }
    }
    Ok(data_list)
}

//チャンク処理
async fn process_chunk(chunk: &[Vec<String>]) {
    let tasks: Vec<_> = chunk
        .iter()
        .flat_map(|inner_vec| {
            inner_vec
                .iter()
                .map(|item| {
                    // 各Stringを並列処理するタスクを生成
                    let item_clone = item.clone();
                    task::spawn(async move {
                        //println!("{}", item_clone);
                        reqwest_status(item_clone).await.unwrap();
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect();

    // 全てのタスクが完了するのを待機
    for task in tasks {
        task.await.unwrap();
    }

    cmd!(green_line); //1チャンクの処理が終わるたびに緑の線を表示する
    tokio::time::sleep(std::time::Duration::from_millis(1760)).await;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup();

    let path = "./sample_base.json";

    let data_list = read_and_print_json(path)?;

    let total_tasks = data_list.iter().map(|data| data.sub.len()).sum::<usize>(); // 全体のタスク数を計算

    let mut completed_tasks = 0; // 完了したタスク数を初期化

    for data in &data_list {
        for chunk in data.sub.chunks(1) {
            process_chunk(chunk).await;
            completed_tasks += chunk.len(); // 完了したタスク数を更新
            print!("進行度: {}/{}", completed_tasks, total_tasks); // 進行度を表示
        }
    }

    //Jsonの構造をStringに変換
    let json_str = serde_json::to_string_pretty(&data_list)?;

    //Jsonをファイルに書き込む
    std::fs::write("output.json", json_str)?;

    Ok(())
}

async fn reqwest_status(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let cookie="NID=516=B5sTLD9MmpyVULQJ6AmqbpmxIpfOIvG_APdFtnGFgXa6Z7nrM8V1DZUFnZY85C5I9hSjOM_dZDwluyrFERQSYyYPO9d_s_X9zWBTQfuWJD6H7g1gIFp83j2NorXNlanMaOBwqP5YKK3PeiVcmbJ1ZyUrFEYhbLQZR4EsCE1_yr8t-LhCdw22c7hGE2YbsQ; AEC=AVYB7cp9JojNVxZx59vxpczaCi7I8_Gt4TIQjCPLmcpR92ZIsLUAFlutGO8; OGPC=19037049-1:";
    let client = reqwest::Client::builder().build()?;
    let response = client
        .get(&url)
        .header("Cookie", cookie)
        //.header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
        .send()
        .await?;
    let status = response.status();
    if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
        cmd!(red_line);
        cmd!(red_line);
        cmd!(red_line);
        cmd!(red_line);
        std::process::exit(0);
    }
    println!("HTTPSステータス: {}", status);
    Ok(())
}
