use sqlx::{postgres::PgPool, Error, FromRow};
use futures::TryStreamExt; // access to try_next

#[derive(FromRow)]
struct Example {
    id: i32,
    name: String,
}

#[derive(FromRow)]
struct Joined {
    id: i32,
    email: String,
    example_id: i32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // TODO: Don't return Error. Handle it.
    let pool = PgPool::connect("postgres://postgres:mypass@localhost:5432/postgres").await?;
    let mut records = sqlx::query_as::<_, Example>("SELECT * FROM example").fetch(&pool);

    // TODO: Handle error cases
    while let Some(record) = records.try_next().await? {
        println!("{} {}", record.id, record.name);
    }

    Ok(())
}
