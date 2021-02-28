use std::error::Error;
use arsan::config::Config;
use arsan::athan::Athan;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let configuration = Config::new("Rabat", "Morocco");
    let athan = Athan::new(configuration).await?;
    println!("{:?}", athan.timings);
    Ok(())
}
