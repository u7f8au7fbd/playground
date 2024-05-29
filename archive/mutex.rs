#[macro_use]
mod macros;

use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;

struct Test {
    string: String,
    int: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = Arc::new(Mutex::new(Test {
        string: "String".to_string(),
        int: 2,
    }));
    cmd!(clear); //画面クリア
    cmd!(utf - 8); //ターミナルをUTF-8へ
    cmd!(line); //開始ライン

    test_read(data.clone()).await?;
    let data_clone = data.clone();
    test_write1(data_clone).await?;

    test_read(data.clone()).await?;
    let data_clone = data.clone();
    test_write2(data_clone).await?;
    Ok(())
}

async fn test_read(data: Arc<Mutex<Test>>) -> Result<(), Box<dyn std::error::Error>> {
    let data = data.lock().await;
    println!("{}:{}", data.string, data.int);
    Ok(())
}

async fn test_write1(data: Arc<Mutex<Test>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut data = data.lock().await;
    *data = Test {
        string: "Mutex1".to_string(),
        int: 1,
    };
    Ok(())
}

async fn test_write2(data: Arc<Mutex<Test>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut data = data.lock().await;
    *data = Test {
        string: "Mutex2".to_string(),
        int: 2,
    };
    Ok(())
}
