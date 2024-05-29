use std::process::Command;
#[macro_use]
mod macros;

fn main() {
    cmd!(utf - 8);
    let output = Command::new("powershell")
                        .args(["-Command", "Invoke-WebRequest -Uri \"https://woman-type.jp\" -OutFile \"C:\\Users\\u7f8a\\Downloads\\download.html\""])
                        .output()
                        .expect("コマンドの実行に失敗しました");

    if output.status.success() {
        println!("コマンドは正常に実行されました");
    } else {
        eprintln!("コマンドの実行に失敗しました: {:?}", output);
    }
}
