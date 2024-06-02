#[macro_use]
mod macros;
use std::process::Command;

fn setup() {
    cmd!(clear); // clearコマンドを実行する
    cmd!(utf - 8); // utf-8コマンドを実行する
    cmd!(red_line); // lineコマンドを実行する
}

fn main() {
    setup();
    get_rnking("rust", 0);
}

fn format_query(query: &str) -> String {
    query.replace(' ', "")
}

fn get_rnking(query: &str, page: i32) {
    let query = format_query(query);
    let url = format!("https://www.google.com/search?q={}&start={}", query, page);
}

fn print_query(query: &str) {
    println!("検索クエリ: {}", query);
}