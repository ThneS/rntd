use reqwest::blocking::Client;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.get("https://httpbin.org/get").send()?;
    println!("{:?}", res.text()?);
    Ok(())
}
