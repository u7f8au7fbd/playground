use reqwest::Client;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

async fn download_html(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).header("Cookie", "").send().await?;
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)
        .await?;

    let content = response.text().await?;
    file.write_all(content.as_bytes()).await?;
    Ok(())
}

async fn get_html_status(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).header("Cookie", "").send().await?;
    println!("HTML status: {}", response.status());
    Ok(())
}

async fn get_html_content(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).header("Cookie", "").send().await?;
    let content = response.text().await?;
    println!("HTML content:\n{}", content);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "./test/test.html";
    let url = "https://www.netflix.com/";

    for _ in 0..100 {
        get_html_status(url).await?;
    }

    Ok(())
}
