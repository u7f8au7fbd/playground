#[derive(Debug, serde::Serialize)]
struct QueryData {
    main: String,
    sub: Vec<Vec<String>>,
}
use serde_json::Value;

#[macro_use]
mod macros;

fn setup() {
    cmd!(clear); // clearコマンドを実行する
    cmd!(utf8); // utf-8コマンドを実行する
    cmd!(red_line); // lineコマンドを実行する
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    let path = "./sample.json";
    let data_list = read_and_print_json(path)?;
    println!("{:#?}", data_list);

    // Serialize data_list to JSON
    let json_str = serde_json::to_string_pretty(&data_list)?;

    // Write JSON to output file
    std::fs::write("output.json", json_str)?;
    Ok(())
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
