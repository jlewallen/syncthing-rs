use syncthing::{Client, Fallible};

#[tokio::main]
async fn main() -> Fallible<()> {
    let client = Client::new(include_str!("../api.key"));
    let system = client.get_system_version().await?;
    println!("syncthing {} is running on {}!", system.version, system.os);

    let connections = client.get_system_connections().await?;
    for (key, device) in connections.connections {
        println!("{} {:?}", key, device);
    }

    Ok(())
}
