#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        div{id :"links",
        table{
            for i in 0..10{
                div{a{href:"https://www.google.co.jp/search?q=Rust&start={i*10}", "Rust:{i*10}" }}
            }
        }
    }
        div{id :"tables",
            //5x5のテーブルを描画
            table{
                for i in 1..=100{
                    tr{
                        for j in 1..100{
                            td{"{i*j}"}
                        }
                    }
                }
            }
        }
    }
}
