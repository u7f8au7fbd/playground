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

    let test = generate_array(1, 30);


    let chunk_size = 10;

    // 配列をチャンクに分割し、非同期処理を行う
    process_in_chunks(&test, chunk_size).await?;

    Ok(())
}

async fn process_in_chunks(
    data: &[i32],
    chunk_size: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    for chunk in data.chunks(chunk_size) {
        let mut tasks = Vec::new();
        for &number in chunk {
            let task = tokio::spawn(async move {
                //numberは配列の内容を示す.
                print(number).await;
            });
            tasks.push(task);
        }

        // 各チャンクのタスクがすべて完了するのを待つ
        for task in tasks {
            task.await?;
        }
        cmd!(green_line); // lineコマンドを実行する
    }
    Ok(())
}

async fn print(num: i32) {
    println!("{}", num.pow(2));
}

fn generate_array(min: i32, max: i32) -> Vec<i32> {
    let mut array = Vec::new();
    for num in min..=max {
        array.push(num);
    }
    array
}
