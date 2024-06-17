#[macro_use]
mod macros;

fn setup() {
    cmd!(clear); // clearコマンドを実行する
    cmd!(utf8); // utf-8コマンドを実行する
    cmd!(red_line); // lineコマンドを実行する
}
use reqwest;
use tokio;
use select::document::Document;
use select::predicate::Name;
use url::Url;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    // 指定されたURLからHTMLをダウンロード
    let url = "https://www.rust-lang.org"; // ここにダウンロードしたいWebページのURLを入力
    let res = reqwest::get(url).await?;
    let body = res.text().await?;

    // HTMLファイルとして保存
    let html_path = Path::new("index.html");
    fs::write(html_path, &body)?;

    // HTMLを解析してリソースのURLを見つける
    let document = Document::from(body.as_str());
    let base_url = Url::parse(url)?;

    // CSSファイルのURLを見つけてダウンロード
    for node in document.find(Name("link")) {
        if let Some(href) = node.attr("href") {
            if href.ends_with(".css") {
                let css_url = base_url.join(href)?;
                let mut res = reqwest::get(css_url.as_str()).await?;
                let mut out = File::create(Path::new(href))?;
                while let Some(chunk) = res.chunk().await? {
                    out.write_all(&chunk)?;
                }
            }
        }
    }

    // JavaScriptファイルのURLを見つけてダウンロード
    for node in document.find(Name("script")) {
        if let Some(src) = node.attr("src") {
            let js_url = base_url.join(src)?;
            let mut res = reqwest::get(js_url.as_str()).await?;
            let mut out = File::create(Path::new(src))?;
            while let Some(chunk) = res.chunk().await? {
                out.write_all(&chunk)?;
            }
        }
    }

    // 画像ファイルのURLを見つけてダウンロード
    for node in document.find(Name("img")) {
        if let Some(src) = node.attr("src") {
            let img_url = base_url.join(src)?;
            let mut res = reqwest::get(img_url.as_str()).await?;
            let mut out = File::create(Path::new(src))?;
            while let Some(chunk) = res.chunk().await? {
                out.write_all(&chunk)?;
            }
        }
    }

    Ok(())
}
