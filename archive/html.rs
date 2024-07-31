use scraper::{ElementRef, Html, Selector};
use std::fs;

#[macro_use]
mod macros;

fn main() {
    cmd!(clear);
    // ファイルの内容を読み取る
    let file_content = fs::read_to_string("./sample/index.html").expect("Failed to read file");

    // HTMLドキュメントを解析する
    let document = Html::parse_document(&file_content);

    // 対象とするタグのセレクタを作成する
    let selectors = vec![
        ("h1", Selector::parse("h1").unwrap()),
        ("h2", Selector::parse("h2").unwrap()),
        ("h3", Selector::parse("h3").unwrap()),
        ("p", Selector::parse("p").unwrap()),
        ("a", Selector::parse("a").unwrap()),
    ];

    // 要素を保持するためのベクタを初期化する
    let mut elements: Vec<(String, String)> = Vec::new();

    // ノードのツリー構造を利用して全ての要素を順番に走査
    for node in document.tree.nodes() {
        if let Some(element) = ElementRef::wrap(node) {
            for (tag_name, _) in &selectors {
                if element.value().name() == *tag_name {
                    let text = element.text().collect::<String>().trim().to_string();
                    if !text.is_empty() {
                        elements.push((tag_name.to_string(), text));
                    }
                }
            }
        }
    }

    // 取得した要素を表示する
    for (tag, text) in elements {
        if tag == "p" {
            println!("{}:{}", tag, text);
        } else if tag == "h1" {
            println!("{}:{}", yellow!(tag), yellow!(text));
        } else if tag == "h2" {
            println!("{}:{}", cyan!(tag), cyan!(text));
        } else if tag == "h3" {
            println!("{}:{}", magenta!(tag), magenta!(text));
        } else if tag == "a" {
            println!("{}:{}", blue!(tag), blue!(text));
        }
    }

    let text = "aaa";

    println!("{}", black!(text));
}
