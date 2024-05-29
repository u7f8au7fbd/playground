#[macro_use]
mod macros;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let mut tasks = Vec::new();
    for run in test.iter() {
        let task = tokio::spawn(print(format!("{}", run)));
        tasks.push(task);
    }

    for task in tasks {
        task.await?;
    }

    Ok(())
}

async fn print(msg: String) {
    println!("{}", msg);
}
