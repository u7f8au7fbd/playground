async fn download_html(url: String) {
    let file_path = "./db/";
    let file_name = url.split('/').last().unwrap();
    let file_path = format!("{}{}.html", file_path, file_name);
    let url_clone = url.clone();
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3",
        )
        .send()
        .await;
    match res {
        Ok(response) => {
            let body = response.text().await.unwrap();
            std::fs::write(file_path, body).unwrap();
            println!(
                "{}Downloaded:{} {}",
                cmd_color!(cyan_b),
                cmd_color!(reset),
                url_clone
            );
        }
        Err(error) => {
            println!(
                "{}Download Error: {}{}",
                cmd_color!(red_b),
                error,
                cmd_color!(reset)
            );
        }
    }
}

async fn connect_test(url: String) -> bool {
    let url_clone = url.clone();
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3",
        )
        .send()
        .await;
    match res {
        Ok(response) => {
            println!(
                "{}{}: {}:{}",
                cmd_color!(green_b),
                response.status(),
                cmd_color!(reset),
                url_clone
            );
            download_html(url_clone).await;
            true
        }
        Err(error) => {
            println!(
                "{}Connect Error:{}{}",
                cmd_color!(red_b),
                error,
                cmd_color!(reset)
            );
            false
        }
    }
}
