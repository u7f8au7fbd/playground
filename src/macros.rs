//マクロを指定
#[macro_export]

macro_rules! cmd_color {
    (black) => {
        "\x1b[30m"
    };
    (red) => {
        "\x1b[31m"
    };
    (green) => {
        "\x1b[32m"
    };
    (yellow) => {
        "\x1b[33m"
    };
    (blue) => {
        "\x1b[34m"
    };
    (magenta) => {
        "\x1b[35m"
    };
    (cyan) => {
        "\x1b[36m"
    };
    (white) => {
        "\x1b[37m"
    };
    (black_b) => {
        "\x1b[40m"
    };
    (red_b) => {
        "\x1b[41m"
    };
    (green_b) => {
        "\x1b[42m"
    };
    (yellow_b) => {
        "\x1b[43m"
    };
    (blue_b) => {
        "\x1b[44m"
    };
    (magenta_b) => {
        "\x1b[45m"
    };
    (cyan_b) => {
        "\x1b[46m"
    };
    (white_b) => {
        "\x1b[47m"
    };
    (reset) => {
        "\x1b[0m"
    };
}

#[macro_export]
macro_rules! cmd {
    (utf8) => {
        use std::process::Command;
        Command::new("cmd")
            .args(["/C", "chcp 65001"])
            .output()
            .expect("UTF-8に設定できませんでした");
    };
    (clear) => {
        Command::new("cmd")
            .args(["/C", "cls"])
            .output()
            .expect("コンソールをリセットできませんでした");
    };
    (line) => {
        println!("----------------------------------------------------------------")
    };
    (red_line) => {
        println!(
            "{}----------------------------------------------------------------{}",
            cmd_color!(red),
            cmd_color!(reset)
        )
    };
    (green_line) => {
        println!(
            "{}----------------------------------------------------------------{}",
            cmd_color!(green),
            cmd_color!(reset)
        )
    };
}

#[macro_export]
macro_rules! format_path {
    ($path:expr) => {
        $path.replace(
            |c: char| {
                !c.is_ascii_alphanumeric()
                    && c != '\\'
                    && c != '/'
                    && c != '<'
                    && c != '>'
                    && c != ':'
                    && c != '?'
                    && c != '*'
                    && c != '|'
                    && c != '"'
            },
            "",
        )
    };
}

#[macro_export]
macro_rules! madogen {
    () => {};
}

#[macro_export]
macro_rules! ini_dir {
    ($path:expr) => {
        use std::{fs, path};
        let exists = path::Path::new($path).exists();
        if exists {
            fs::remove_dir_all($path).expect("ディレクトリを削除できませんでした");
            fs::create_dir_all($path).expect("ディレクトリを作成できませんでした");
        } else {
            fs::create_dir_all($path).expect("ディレクトリを作成できませんでした");
        }
    };
}
