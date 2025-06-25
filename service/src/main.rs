use anyhow::{Error, Result};
use reservation_service::start_server;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = String::from("127.0.0.1:50051");
    start_server(&url).await
}
