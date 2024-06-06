async fn google_query(query: &str, page: u32){
    let mut urls = Vec::new();
    let path= format!("./db/Google/{}",query);
    ini_dir!(&path);
    for page in 0..page  {
        let url = url_format(query, page);
        urls.push(url);
    }
    fn url_format(query: &str,page: u32)->String {
        let query = query.replace(' ', "+");
        let url = format!("https://www.google.com/search?q={}&start={}", query, page);
        url
    }
    download_task(urls,path).await.unwrap();
}

async fn download_task(url_list: Vec<String>,path:String) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = Vec::new();
    // ダウンロードするためのタスクを生成
    for (index, url) in url_list.into_iter().enumerate() {
        let path = path.clone();
        let task = tokio::spawn(async move {
            download(&url, &path, index).await
        });
        tasks.push(task);
    }
    // ダウンロードするためのタスクを実行
    for task in tasks {
        task.await?;
    }
    Ok(())
}


async fn download(url: &str, path: &str, index: usize) {
    let file_path = format!("{}/{}.html", path, index);
    let command = Command::new("powershell")
        .args([&format!(
            "Invoke-WebRequest -Uri \"{}\" -OutFile \"{}\"",
            url, file_path
        )])
        .output();

    if command.unwrap().status.success() {
        println!("{}{}{}", cmd_color!(green_b), index, cmd_color!(reset));
    } else {
        println!("{}{}{}", cmd_color!(red_b), index, cmd_color!(reset));
    }
}

