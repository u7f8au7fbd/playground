#[macro_use]
mod macros;
use scraper::html;
use std::fs::File;
use std::io::Read;
use std::process::Command;

fn setup() {
    cmd!(clear); // clearコマンドを実行する
    cmd!(utf - 8); // utf-8コマンドを実行する
    cmd!(red_line); // lineコマンドを実行する
}

fn main() {
    setup();
    let check_item: Vec<bool> = Vec::new();

    let mut sei: String =
        "https://www.googleapis.com/pagespeedonline/v5/runPagespeed?url=".to_string();
    let url = "https://www.rust-lang.org/ja";
    sei.push_str(url);

    println!("{}", sei);
    get_title_from_html();
}

fn get_title_from_html() {
    let file_path = "./db/HTML/rust.html";
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    let document = html::Html::parse_document(&contents);
    search_and_get(&document, "title");
    cmd!(line);
    print_meta_tags_from_html(&document);
    cmd!(line);
    print_link_tags_from_html(&document);
    cmd!(line);
    get_dom_size_from_html(&document);
    cmd!(line);
    get_description_from_html(&document);
    cmd!(line);
    get_title_and_h1_text(&document);
}

// タグを検索して、その中身を表示する
fn search_and_get(document: &html::Html, tag: &str) -> String {
    let mut result = String::new();
    let h1_elements = document
        .select(&scraper::Selector::parse(tag).unwrap())
        .collect::<Vec<_>>();

    if h1_elements.is_empty() {
        println!("{}が見つかりませんでした", tag);
    } else {
        for elements in h1_elements {
            let element: String = elements
                .text()
                .collect::<String>()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();
            println!("{}: {}", tag, element);
            result = element;
        }
    }
    result
}
// metaタグを取得して、それぞれの属性を表示する
fn print_meta_tags_from_html(document: &html::Html) {
    let binding = scraper::Selector::parse("meta").unwrap();
    let meta_tags = document.select(&binding);
    for tag in meta_tags {
        if let Some(name) = tag.value().attr("name") {
            println!("meta tag with name: {}", name);
        }
        if let Some(property) = tag.value().attr("property") {
            println!("meta tag with property: {}", property);
        }
        if let Some(property) = tag.value().attr("content") {
            println!("meta tag with property: {}", property);
        }
        if let Some(property) = tag.value().attr("charset") {
            println!("meta tag with property: {}", property);
        }
        if let Some(property) = tag.value().attr("http-equiv") {
            println!("meta tag with property: {}", property);
        }
    }
}
// linkタグを取得して、それぞれの属性を表示する
fn print_link_tags_from_html(document: &html::Html) {
    let binding = scraper::Selector::parse("link").unwrap();
    let meta_tags = document.select(&binding);
    for tag in meta_tags {
        if let Some(name) = tag.value().attr("rel") {
            println!("meta tag with name: {}", name);
        }
        if let Some(property) = tag.value().attr("href") {
            println!("meta tag with property: {}", property);
        }
        if let Some(property) = tag.value().attr("type") {
            println!("meta tag with property: {}", property);
        }
        if let Some(property) = tag.value().attr("sizes") {
            println!("meta tag with property: {}", property);
        }
        if let Some(property) = tag.value().attr("hreflang") {
            println!("meta tag with property: {}", property);
        }
    }
}

// DOMのサイズを取得すして、1500未満であればtrueを返す
fn get_dom_size_from_html(document: &html::Html) -> bool {
    let elements = document
        .select(&scraper::Selector::parse("*").unwrap())
        .collect::<Vec<_>>();
    let dom_size = elements.len();
    println!("DOM size: {}", dom_size);
    dom_size < 1500
}

// descriptionタグを取得して、その中身を表示し，その長さを表示する
fn get_description_from_html(document: &html::Html) -> i32 {
    let binding = scraper::Selector::parse("meta[name='description']").unwrap();
    let description_tag = document.select(&binding).next();
    if let Some(tag) = description_tag {
        if let Some(description) = tag.value().attr("content") {
            println!("Description: {}", description);
            let description_length = description.chars().count();
            println!("Description Length: {}", description_length);
            return description_length as i32;
        }
    } else {
        println!("Description tag not found");
        return 0;
    }
    0
}

//Titleとh1タグに異なるテキストを設定
fn get_title_and_h1_text(document: &html::Html) -> bool {
    if search_and_get(document, "title") != search_and_get(document, "h1") {
        println!("タイトルとh1タグのテキストが異なります");
        return true;
    }
    false
}
