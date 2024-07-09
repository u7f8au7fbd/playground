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
    let path = "./test.json";
    read_and_print_json(path)?;
    println!("{:#?}", read_and_print_json(path)?);
    Ok(())
}

fn read_and_print_json(path: &str) -> Result<Vec<Vec<Vec<String>>>, Box<dyn std::error::Error>> {
    let mut list: Vec<Vec<Vec<String>>> = Vec::new(); //最終的に返す配列を定義

    let json_str = std::fs::read_to_string(path)?; //jsonを読み込む

    let json: Value = serde_json::from_str(&json_str)?; //jsonをパースする

    if let Value::Array(arr) = json {
        for obj in arr {
            //配列の中身を取り出す
            if let Value::String(main_word) = obj["main_word"].clone() {
                // main_wordを取り出す
                if let Value::Array(sub_word) = obj["sub_word"].clone() {
                    // sub_wordを取り出す
                    let mut url_list: Vec<Vec<String>> = Vec::new(); // URLを格納するための配列

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
                            url_list.push(page_urls); // ページごとのURLの配列をurl_listに格納
                        }
                    }
                    list.push(url_list); // URLの配列をlistに格納
                }
            }
        }
    }
    Ok(list)
}
