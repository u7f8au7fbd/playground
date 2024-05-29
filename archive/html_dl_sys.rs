use std::process::Command;

#[macro_use]
mod macros;

fn setup() {
    cmd!(clear);
    cmd!(utf - 8);
    cmd!(line)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    download_html().await?;
    Ok(())
}

struct SearchData {
    query: String,
    urls: Vec<[String; 100]>,
}

async fn download_html() -> Result<(), Box<dyn std::error::Error>> {
    let url_list = [
        "https://woman-type.jp/",
        "https://google.co.jp/",
        "https://rust-lang.org",
    ];
    let path: &str = "./db/data/";
    let mut tasks = Vec::new();
    for (index, run) in url_list.iter().enumerate() {
        println!("\n");
        let task = tokio::spawn(download(run, path, index));
        tasks.push(task);
    }

    for task in tasks {
        task.await?;
    }

    async fn download(url: &str, path: &str, index: usize) {
        let file_path = format!("{}{}.html", path, index);
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

    Ok(())
}
