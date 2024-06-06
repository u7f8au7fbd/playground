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
    let url = "https://www.google.com/search?q=Rust&start=0";
    let html_data = reqwest::get(url).await?.bytes().await?;
    std::fs::write("./db/test/test.html", html_data).expect("保存失敗");
    Ok(())
}