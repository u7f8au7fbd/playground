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
    Ok(())
}

