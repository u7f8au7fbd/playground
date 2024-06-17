#[macro_use]
mod macros;

fn setup() {
    cmd!(clear); // clearコマンドを実行する
    cmd!(utf8); // utf-8コマンドを実行する
    cmd!(red_line); // lineコマンドを実行する
}

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// reqwestクレートとselectクレートを使用してHTMLを解析し、リソースを取得します。
use reqwest;
use select::document::Document;
use select::predicate::Name;

// 指定されたURLからHTMLをダウンロードし、リソースのURLを抽出します。
async fn download_resources(url: &str) -> Result<(), Box<dyn Error>> {
    // HTMLを取得します。
    let res = reqwest::get(url).await?.text().await?;

    // ドキュメントを解析します。
    let document = Document::from(res.as_str());

    // CSSファイルのリンクを抽出します。
    for node in document.find(Name("link")) {
        if let Some(href) = node.attr("href") {
            if href.ends_with(".css") {
                download_file(href).await?;
            }
        }
    }

    // 画像ファイルのリンクを抽出します。
    for node in document.find(Name("img")) {
        if let Some(src) = node.attr("src") {
            download_file(src).await?;
        }
    }

    Ok(())
}

// 指定されたURLからファイルをダウンロードし、ローカルに保存します。
async fn download_file(url: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(url).await?;

    // URLからファイル名を抽出します。
    let filename = Path::new(url)
        .file_name()
        .ok_or("Could not extract file name")?
        .to_str()
        .ok_or("Could not convert file name to string")?;

    // ファイルを作成します。
    let mut file = File::create(filename)?;

    // ファイルにコンテンツを書き込みます。
    let content = response.bytes().await?;
    file.write_all(&content)?;

    Ok(())
}

#[tokio::main]
async fn main() {
    // ダウンロードしたいWebページのURLを指定します。
    let url = "https://www.example.com";
    setup();
    // リソースをダウンロードします。
    if let Err(e) = download_resources(url).await {
        println!("エラーが発生しました: {}", e);
    }
}
