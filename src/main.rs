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
    Ok(())
}

fn read_and_print_json(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json_str = std::fs::read_to_string(path)?;
    let json: Value = serde_json::from_str(&json_str)?;
    if let Value::Array(arr) = json {
        for obj in arr {
            if let Value::String(main_word) = obj["main_word"].clone() {
                if let Value::Array(sub_word) = obj["sub_word"].clone() {
                    for word in sub_word {
                        if let Value::String(word) = word {
                            println!("{}+{}", main_word, word);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
