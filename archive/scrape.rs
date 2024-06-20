use std::fs;

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
    scrape_title();
    cmd!(green_line);
    print_href();
    cmd!(green_line);
    print_img_src();
    cmd!(green_line);
    print_script_src();
    Ok(())
}

fn scrape_title() {
    let html = fs::read_to_string("./sample/index.html").expect("Failed to read HTML file");
    let document = scraper::Html::parse_document(&html);
    let title = document
        .select(&scraper::Selector::parse("title").unwrap())
        .next();

    if let Some(title) = title {
        let title_text = title.text().collect::<String>();
        println!("Title: {}", title_text);
    } else {
        println!("No title found");
    }
}
fn print_href() {
    let html = fs::read_to_string("./sample/test/index.html").expect("Failed to read HTML file");
    let document = scraper::Html::parse_document(&html);
    let binding = scraper::Selector::parse("head link[href]").unwrap();
    let hrefs = document.select(&binding);
    for href in hrefs {
        let href_value = href.value().attr("href").unwrap();
        println!("Href: {}", href_value);
    }
}

fn print_img_src() {
    let html = fs::read_to_string("./sample/test/index.html").expect("Failed to read HTML file");
    let document = scraper::Html::parse_document(&html);
    let binding = scraper::Selector::parse("img[src]").unwrap();
    let srcs = document.select(&binding);
    for src in srcs {
        let src_value = src.value().attr("src").unwrap();
        println!("Img Src: {}", src_value);
    }
}
fn print_script_src() {
    let html = fs::read_to_string("./sample/test/index.html").expect("Failed to read HTML file");
    let document = scraper::Html::parse_document(&html);
    let binding = scraper::Selector::parse("script[src]").unwrap();
    let srcs = document.select(&binding);
    for src in srcs {
        let src_value = src.value().attr("src").unwrap();
        println!("Script Src: {}", src_value);
    }
}
