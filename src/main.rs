
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
