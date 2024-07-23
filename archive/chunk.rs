#[macro_use]
mod macros;

fn setup() {
    cmd!(clear); // clearコマンドを実行する
    cmd!(utf8); // utf-8コマンドを実行する
    cmd!(red_line); // lineコマンドを実行する
}

use tokio::runtime::Runtime;
use tokio::task;

struct Data {
    string: String,
    data: Vec<Vec<String>>,
}

async fn process_chunk(chunk: &[Vec<String>]) {
    let tasks: Vec<_> = chunk
        .iter()
        .flat_map(|inner_vec| {
            inner_vec
                .iter()
                .map(|item| {
                    // 各Stringを並列処理するタスクを生成
                    let item_clone = item.clone();
                    task::spawn(async move {
                        println!("{}", item_clone);
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect();

    // 全てのタスクが完了するのを待機
    for task in tasks {
        task.await.unwrap();
    }
    println!("チャンクが完了しました。");
}

#[tokio::main]
async fn main() {
    let mut test_data: Vec<Data> = Vec::new();

    // 10x10x10のデータを生成
    for i in 0..3 {
        let mut data = Vec::new();
        for j in 0..100 {
            let mut row = Vec::new();
            for k in 0..10 {
                row.push(format!("{}{}{}", i, j, k));
            }
            data.push(row);
        }
        test_data.push(Data {
            string: i.to_string(),
            data,
        });
    }

    // 基盤的な配列を最小単位として並列処理
    for data in &test_data {
        for chunk in data.data.chunks(1) {
            process_chunk(chunk).await;
        }
    }
    println!("全ての処理が完了しました。");
}
