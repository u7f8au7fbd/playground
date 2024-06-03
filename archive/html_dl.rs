use std::process::Command;

#[macro_use]
mod macros;

fn setup() {
    cmd!(clear);
    cmd!(utf8);
    cmd!(line)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    let url_list = [
        "https://www.google.co.jp/search?q=Rust&start=0",
        "https://www.google.co.jp/search?q=Rust&start=10",
        "https://www.google.co.jp/search?q=Rust&start=20",
        "https://www.google.co.jp/search?q=Rust&start=30",
        "https://www.google.co.jp/search?q=Rust&start=40",
        "https://www.google.co.jp/search?q=Rust&start=50",
        "https://www.google.co.jp/search?q=Rust&start=60",
        "https://www.google.co.jp/search?q=Rust&start=70",
        "https://www.google.co.jp/search?q=Rust&start=80",
        "https://www.google.co.jp/search?q=Rust&start=90",
        ];
        download_task(url_list).await?;
    Ok(())
}

async fn download_task<'a>(url_list: [&'a str; 10]) -> Result<(), Box<dyn std::error::Error + 'a>> {
    let path = "./db/Data/".to_string();
    ini_dir!(&path);
    let mut tasks = Vec::new();
    //ダウンロードするためのタスクを生成
    for (index, &url) in url_list.iter().enumerate() {
        let path = path.clone();
        let url = url.to_string();
        let task = tokio::spawn(async move {
            download(&url, &path, index).await
        });
        tasks.push(task);
    }
    //ダウンロードするためのタスクを実行
    for task in tasks {
        task.await?;
    }
    Ok(())
}

async fn download(url: &str, path: &str, index: usize) {
    let file_path = format!("{}{}.html", path, index);
    ini_dir!(&file_path);
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