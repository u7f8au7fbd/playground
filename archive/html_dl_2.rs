async fn download_html(url: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // PowerShellでダウンロードする関数
    async fn download_html_ps(url: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = format!("{}index1.html", path);
        Command::new("powershell")
            .args([&format!(
                "Invoke-WebRequest -Uri \"{}\" -OutFile \"{}\"",
                url, file_path
            )])
            .output()?;
        Ok(())
    }

    let file_path = format!("{}index2.html", path);
    let client = Client::builder().timeout(Duration::from_secs(3)).build()?;
    let response = client.get(url).send().await;
    match response {
        Ok(response) => {
            let body = response.text().await?;
            std::fs::write(file_path, body)?;
            Ok(())
        }
        Err(_) => {
            download_html_ps(url, path).await?;
            Ok(())
        }
    }
}
