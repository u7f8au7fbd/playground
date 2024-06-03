#[macro_use]
mod macros;

fn setup() {
    cmd!(clear);
    cmd!(utf8);
    cmd!(line)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup();
    Ok(())
}

fn googling(query: &str,page: u32)->String {
    let _ = query.replace(' ', "+");
    let query = "rust lang";
    let url = format!("https://www.google.com/search?q={}&start={}", query, page);
    url
}