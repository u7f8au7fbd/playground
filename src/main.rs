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
        tokio::time::sleep(std::time::Duration::from_millis(1760)).await;
    }
}

#[tokio::main]
async fn main() {
    setup();
    time_count!({
        // `String`のベクターを作成します。
        let num_str = fill_vec("https://www.google.com/", 10000);
        let chunk_size = 10;
        // `Vec<String>`を渡して所有権の問題を回避します。
        process_in_chunks(num_str, chunk_size).await;
    });
}

fn fill_vec(string: &str, size: usize) -> Vec<String> {
    let mut vec = Vec::with_capacity(size);
    for _ in 0..size {
        vec.push(string.to_owned());
    }
    vec
}

//クッキーを追加
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
