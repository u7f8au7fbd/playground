#[macro_use]
mod macros;
fn setup() {
    cmd!(clear); // clearコマンドを実行する
    cmd!(utf8); // utf-8コマンドを実行する
    cmd!(red_line); // lineコマンドを実行する
}
use tokio::task;

async fn process_in_chunks(num_str: Vec<String>, chunk_size: usize) {
    let mut counter = 0;
    for chunk in num_str.chunks(chunk_size) {
        let mut handles = Vec::new();
        for num in chunk {
            // `to_owned()`を使用して、参照ではなくデータの所有権を持つクローンを作成します。
            let num = num.to_owned();
            let handle = task::spawn(async move {
                reqwest_status(num).await.unwrap();
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
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
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}

#[tokio::main]
async fn main() {
    setup();
    // `String`のベクターを作成します。
    let num_str = fill_vec("https://www.google.co.jp/", 1000);
    let chunk_size = 100;

    // `Vec<String>`を渡して所有権の問題を回避します。
    process_in_chunks(num_str, chunk_size).await;
}

fn fill_vec(string: &str, size: usize) -> Vec<String> {
    let mut vec = Vec::with_capacity(size);
    for _ in 0..size {
        vec.push(string.to_owned());
    }
    vec
}

async fn reqwest_status(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let status = reqwest::get(url).await?.status();
    println!("HTTPSステータス: {}", status);
    Ok(())
}
