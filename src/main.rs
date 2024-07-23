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
                        println!("{}", item_clone);
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect();

    // 全てのタスクが完了するのを待機
    for task in tasks {
        task.await.unwrap();
    }
    println!("チャンクが完了しました。");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup();

    let path = "./sample_base.json";
    let data_list = read_and_print_json(path)?;
    for data in &data_list {
        for chunk in data.sub.chunks(1) {
            process_chunk(chunk).await;
        }
    }

    //Jsonの構造をStringに変換
    let json_str = serde_json::to_string_pretty(&data_list)?;

    //Jsonをファイルに書き込む
    std::fs::write("output.json", json_str)?;

    Ok(())
}
