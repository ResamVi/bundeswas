use dip;
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

    let bundestag = dip::new();

    for akt in bundestag.aktivitaeten() {
        println!("{}", akt.id);

        let fund = akt.fundstelle;
        sqlx::query!(r#"
            INSERT INTO fundstellen(pdf_url, id, dokumentnummer, datum, dokumentart, drucksachetyp, herausgeber, aktivitaeten_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8) ON CONFLICT DO NOTHING
        "#, fund.pdf_url, fund.id, fund.dokumentnummer, fund.datum, fund.dokumentart, fund.drucksachetyp, fund.herausgeber, akt.id).execute(&pool).await?;

        sqlx::query!(r#"
            INSERT INTO aktivitaeten(id, aktivitaetsart, typ, vorgangsbezug_anzahl, dokumentart, wahlperiode, datum, titel, fundstelle_id) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#, akt.id, akt.aktivitaetsart, akt.typ, akt.vorgangsbezug_anzahl, akt.dokumentart, akt.wahlperiode, akt.datum, akt.titel, fund.id).execute(&pool).await?;

        for urheber in fund.urheber.iter() {
            sqlx::query!(r#"
                INSERT INTO fundstellen_urheber(fundstelle_id, urheber) 
                VALUES ($1, $2) 
            "#, fund.id, urheber).execute(&pool).await?;
        }

        if let Some(vorgangsbezuege) = akt.vorgangsbezug {
            for v in vorgangsbezuege.iter() {
                sqlx::query!(r#"
                    INSERT INTO vorgangsbezuege(id, aktivitaeten_id, vorgangsposition, vorgangstyp, titel) 
                    VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING
                "#, v.id, akt.id, v.vorgangsposition, v.vorgangstyp, v.titel).execute(&pool).await?;
            }
        }
    };


    // TODO: Handle error cases
    // while let Some(record) = records.try_next().await? {
    //     println!("{} {}", record.id, record.name);
    // }

    Ok(())
}
