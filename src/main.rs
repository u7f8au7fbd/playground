use std::process::Command;
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
    reqwest_dl("https://www.google.com/search?q=Rust&start=0","./db/test/test_ps.html").await?;
    powershell_dl("https://www.google.com/search?q=Rust&start=0","./db/test/test_ps.html").await?;
    Ok(())
}

async fn reqwest_dl(url:&str,path:&str)->Result<(), Box<dyn std::error::Error>>{
    let html_data = reqwest::get(url).await?.bytes().await?;
    std::fs::write(path, html_data).expect("保存失敗");
    Ok(())
}


async fn powershell_dl(url: &str, path: &str)->Result<(), Box<dyn std::error::Error>>{

    Ok(())
}