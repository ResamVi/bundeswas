use dip;
use sqlx::{postgres::PgPool, Error, FromRow};
use futures::TryStreamExt; // access to try_next

// TODO: create store crate with store::aktivitaet()
mod aktivitaeten;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // TODO: Don't return Error. Handle it.
    let pool = PgPool::connect("postgres://postgres:mypass@localhost:5432/postgres").await?;

    let bundestag = dip::new();

    // Aktivitäten importieren
    // for akt in bundestag.aktivitaeten().take(5) {
    //     aktivitaeten::store(&pool, akt).await;
    // };


    // TODO: Handle error cases
    // while let Some(record) = records.try_next().await? {
    //     println!("{} {}", record.id, record.name);
    // }

    Ok(())
}
