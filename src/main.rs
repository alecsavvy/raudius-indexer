use std::error::Error;

mod entity_manager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    Ok(())
}
