//! This is a simple example to demonstrate the usage of this library.

#[tokio::main]
async fn main() -> Result<(), minreq::Error> {
    let response = minreq::get("http://example.com").send().await?;
    let html = response.as_str()?;
    println!("{}", html);
    Ok(())
}
