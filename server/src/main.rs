use sqlx::{postgres::PgPool, Error, FromRow, Row};
use futures::TryStreamExt;

#[derive(FromRow)]
struct Example {
    id: i32,
    name: String,
}

// TODO: What is an async function?
#[tokio::main]
async fn main() -> Result<(), Error> {
    // TODO: Await required?
    // TODO: Don't return Error. Handle it.
    let pool = PgPool::connect("postgres://postgres:mypass@localhost:5432/postgres").await?;
    let mut records = sqlx::query("SELECT * FROM example").fetch(&pool);

    while let Some(record) = records.try_next().await? {
        let id: i32 = record.try_get("id")?;
        let name: String = record.try_get("name")?;
        println!("{} {}", id, name);
    }

    Ok(())
}
