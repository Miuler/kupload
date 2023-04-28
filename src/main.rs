use kupload::domain::entity::error::Result;
use kupload::infrastructure::server::ServerBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    let server1 = ServerBuilder::build_with_default().await?;
    println!("server1: {:?}", server1);

    let server2 = server1.builder().build().await?;
    println!("server2: {:?}", server2);

    server2.run().await?;

    println!("TODO OK!!!!");
    Ok(())
}

