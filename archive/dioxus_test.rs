#![allow(non_snake_case)]
use dioxus::desktop::WindowEvent;
use dioxus::prelude::*;
fn main() {
    dioxus::launch(App);
}

fn App() -> Element {
    rsx! {
        iframe { src: "https://www.google.co.jp" }
        link { rel: "stylesheet", href: "main.css" }
        div { id: "links",
            table {
                for i in 0..10 {
                    div {
                        a { href: "https://www.google.co.jp/search?q=Rust&start={i*10}",
                            "Rust:{i*1}"
                        }
                    }
                }
            }
        }
        div { id: "tables",
            //5x5のテーブルを描画
            table {
                for i in 1..=100 {
                    tr {
                        for j in 1..100 {
                            td { "{i*j}" }
                        }
                    }
                }
            }
        }
    }
}
