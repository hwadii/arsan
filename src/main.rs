use arsan::athan::Athan;
use arsan::config::Config;
use std::error::Error;
use std::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let configuration = Config::new("Rabat", "Morocco");
    let athan = Athan::new(configuration).await?;
    let today = athan.tomorrow().unwrap_or_else(|err| {
        eprintln!("There was an error fetching the timing: {}", err);
        process::exit(1);
    });
    println!("{}", today);
    Ok(())
}
