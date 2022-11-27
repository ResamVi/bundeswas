use log::{error, info};
use neo4rs::*;
use futures::stream::*;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = config()
        .uri("127.0.0.1:7687")
        .user("neo4j") // TODO: env vars
        .password("neo") // TODO: Environment Variables
        .db("neo4j") // TODO: change name
        .fetch_size(200) // TODO: magic number. Seems to be recommended
        .max_connections(10) // TODO: magic number
        .build()
        .unwrap();

    // TODO: Idiomatic Rust Error handling
    // TODO: Idiomatic Rust logging
    let graph = Graph::connect(config).await.unwrap();
    info!("Connected to database.");

    let mut result = graph.execute(query("RETURN 1")).await.unwrap();
    info!("Executed query");

    let row = result.next().await.unwrap().unwrap();
    info!("taking result");

    let value: i64 = row.get("1").unwrap();
    info!("parsing result");

    assert!(result.next().await.unwrap().is_none());
    assert_eq!(1, value);
}
