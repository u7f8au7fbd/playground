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
    let mut counter = 0;
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

    counter += 1;
    print!(
        "{}{}/{}{}",
        cmd_color!(green),
        counter,
        num_str.len() / chunk_size,
        cmd_color!(reset)
    );
    cmd!(green_line); //1チャンクの処理が終わるたびに緑の線を表示する
    tokio::time::sleep(std::time::Duration::from_millis(1760)).await;
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

async fn reqwest_status(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;
    let response = client.get(&url)
        .header("Cookie", "NID=515=mlGvH_06_7rOahVHX4mfBgoAnHNDr6C9XXyZkmdo2TSczwJSNVkfMoEGboxtcOXyGUo5PWN8L10cPBn2bfQ3uF4xl6D8UTwz8iH3ppGwEaWuNKd9uwVo6MAUkSj9typIaoihflNSuzPa644wg1OufpcGCzzOfWbVzenEHEcYdFZpzeOqJCpOfDefYncaNrsX-l8QJWOTFlw11khb-sNxV1_WasYZJUJAxRuZnwDeNbmnONlWN9W28ZCimS80gcXiAhO5PXjHVxQ530AMNI5L26EZp7L8_TMnIkYLDFh2xzW9v_pKelnxyASOd_lcB2-2jJlKjpwBVgBs2Snr6a1ScO-nGKMAfpt2bWhdSjNvR66d6IVmVBOF23j3AtRg_YNNBBNQUnW6i7aTm_uaZdSCHyq6KSc_mCrk1w--3muMjI9rOnOPnwB_qF-XC-o8EhycXj_J8l5V-jRwSaN2ARNtN1dVRKNLOSJ-9KkY6eF2Grp6oJ_k6Vd3Y7qiCmenxs7_TY7_U0fzn4Vs2YYfp6QQY_POeIkspGNnoUUc0n_oK04dpIp4C3w-ot0lfqUzC1B50RVnUI1a")
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
