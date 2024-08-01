#[derive(Debug, serde::Serialize)]
struct QueryData {
    main: String,
    sub: Vec<String>,
}

#[macro_use]
mod macros;
use serde_json::Value;

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
                            let url =
                                format!("https://www.google.com/search?q={}+{}", main_word, word);
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
        for sub in &data.sub {
            println!("{}", sub);
        }
    }
    // Serialize data_list to JSON
    let json_str = serde_json::to_string_pretty(&data_list)?;
    // Write JSON to output file
    std::fs::write(format!("{}/log.json", &dir), json_str)?;
    Ok(())
}

fn get_now_time() -> String {
    let now = chrono::Local::now();
    now.format("%Y-%m-%d_%H-%M-%S").to_string()
}
