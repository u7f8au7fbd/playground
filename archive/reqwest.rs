#[macro_use]
mod macros;
use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process::Command;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    cmd!(clear);
    cmd!(utf - 8);
    cmd!(line);
    download_html("https://woman-type.jp/").await?;
    get_status_code("https://woman-type.jp/").await?;
    Ok(())
}

async fn download(url: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let body = response.text().await?;

    let document = Html::parse_document(&body);
    let title_selector = Selector::parse("title").unwrap();
    let title = document
        .select(&title_selector)
        .next()
        .unwrap()
        .text()
        .collect::<String>();

    let file_path = "./db/data.html";
    let mut file = File::create(file_path)?;
    file.write_all(body.as_bytes())?;

    println!("Downloaded HTML from {} and saved it to {}", url, file_path);
    println!("Title: {}", title);

    Ok(())
}

async fn download_html(url: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let body = response.text().await?;

    let document = Html::parse_document(&body);
    let title_selector = Selector::parse("title").unwrap();
    let title = document
        .select(&title_selector)
        .next()
        .unwrap()
        .text()
        .collect::<String>();

    let file_path = "./db/data/download.html";
    let mut file = File::create(file_path)?;
    file.write_all(body.as_bytes())?;

    println!("Downloaded HTML from {} and saved it to {}", url, file_path);
    println!("Title: {}", title);

    Ok(())
}

async fn get_status_code(url: &str) -> Result<u16, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}


