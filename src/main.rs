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
    }
}

#[tokio::main]
async fn main() {
    setup();
    time_count!({
        // `String`のベクターを作成します。
        let num_str = fill_vec("https://www.google.com/", 5000);
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

async fn reqwest_status(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;
    let response = client.get(&url)        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7")
    .header("Accept-Encoding", "gzip, deflate, br, zstd")
    .header("Accept-Language", "ja")
    .header("Cache-Control", "max-age=0")
    .header("Cookie", "receive-cookie-deprecation=1; vidbh=11; SEARCH_SAMESITE=CgQIppsB; vidyk=6; HSID=AK2QPB-s0pD21yy8c; SSID=AnVBqx_2uVCCEpDIQ; APISID=EM8aRTiJfycOXJpM/AfWr7_5SG6sv0_WPC; SAPISID=odzEcfoJVjkYpf0G/AyHlFwQ0UAXPlCaTA; __Secure-1PAPISID=odzEcfoJVjkYpf0G/AyHlFwQ0UAXPlCaTA; __Secure-3PAPISID=odzEcfoJVjkYpf0G/AyHlFwQ0UAXPlCaTA; SID=g.a000lAhskGdaZQq5rIuYRQ5GQsmyq1lFHrzp_L96uMhyV8cMgbyv1g5mKfsole0NuK_leE2YJwACgYKAW0SARcSFQHGX2Mi4M_PURiGjB2haffVkZa0YRoVAUF8yKquYkiHgyRAAQYwtv_uy87w0076; __Secure-1PSID=g.a000lAhskGdaZQq5rIuYRQ5GQsmyq1lFHrzp_L96uMhyV8cMgbyvw4YXSDwHQQJctzSrmUiX2gACgYKAVUSARcSFQHGX2Mi1qOiqwSDsSyArlZnc1hMFRoVAUF8yKrt0MOjzFymiajIDn50S9Cd0076; __Secure-3PSID=g.a000lAhskGdaZQq5rIuYRQ5GQsmyq1lFHrzp_L96uMhyV8cMgbyv7ClUwCKpnX2XJaWqKAV00gACgYKAbcSARcSFQHGX2MilYOAVRZKbnIUK0b5bPHSsRoVAUF8yKqpauDAtdgvLv6-sHjNMrRN0076; AEC=AQTF6Hz01iVmVmjrZxcsz_8Zj1IgcbftKLHA_i2ZBg3zV5JNZBbV6WKxxzc; NID=515=mlGvH_06_7rOahVHX4mfBgoAnHNDr6C9XXyZkmdo2TSczwJSNVkfMoEGboxtcOXyGUo5PWN8L10cPBn2bfQ3uF4xl6D8UTwz8iH3ppGwEaWuNKd9uwVo6MAUkSj9typIaoihflNSuzPa644wg1OufpcGCzzOfWbVzenEHEcYdFZpzeOqJCpOfDefYncaNrsX-l8QJWOTFlw11khb-sNxV1_WasYZJUJAxRuZnwDeNbmnONlWN9W28ZCimS80gcXiAhO5PXjHVxQ530AMNI5L26EZp7L8_TMnIkYLDFh2xzW9v_pKelnxyASOd_lcB2-2jJlKjpwBVgBs2Snr6a1ScO-nGKMAfpt2bWhdSjNvR66d6IVmVBOF23j3AtRg_YNNBBNQUnW6i7aTm_uaZdSCHyq6KSc_mCrk1w--3muMjI9rOnOPnwB_qF-XC-o8EhycXj_J8l5V-jRwSaN2ARNtN1dVRKNLOSJ-9KkY6eF2Grp6oJ_k6Vd3Y7qiCmenxs7_TY7_U0fzn4Vs2YYfp6QQY_POeIkspGNnoUUc0n_oK04dpIp4C3w-ot0lfqUzC1B50RVnUI1a")
    .header("Dnt", "1")
    .header("Priority", "u=0, i")
    .header("Sec-Ch-Ua", "\"Not/A)Brand\";v=\"8\", \"Chromium\";v=\"126\", \"Google Chrome\";v=\"126\"")
    .header("Sec-Ch-Ua-Arch", "\"x86\"")
    .header("Sec-Ch-Ua-Bitness", "\"64\"")
    .header("Sec-Ch-Ua-Full-Version", "\"126.0.6478.114\"")
    .header("Sec-Ch-Ua-Full-Version-List", "\"Not/A)Brand\";v=\"8.0.0.0\", \"Chromium\";v=\"126.0.6478.114\", \"Google Chrome\";v=\"126.0.6478.114\"")
    .header("Sec-Ch-Ua-Mobile", "?0")
    .header("Sec-Ch-Ua-Model", "\"\"")
    .header("Sec-Ch-Ua-Platform", "\"Windows\"")
    .header("Sec-Ch-Ua-Platform-Version", "\"19.0.0\"")
    .header("Sec-Ch-Ua-Wow64", "?0")
    .header("Sec-Fetch-Dest", "document")
    .header("Sec-Fetch-Mode", "navigate")
    .header("Sec-Fetch-Site", "none")
    .header("Sec-Fetch-User", "?1")
    .header("Upgrade-Insecure-Requests", "1")
    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
    .send()
    .await?;

    let status = response.status();
    println!("HTTPSステータス: {}", status);
    Ok(())
}
