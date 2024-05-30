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
}
