use sqlx::{postgres::PgPool, Error, FromRow};

use dip::Aktivitaet;

/// Save the entity in a postgres database.
pub async fn store(pool: &PgPool, akt: Aktivitaet) -> Result<(), Error> {
    let fund = akt.fundstelle;
    sqlx::query!(r#"
            INSERT INTO fundstellen(pdf_url, id, dokumentnummer, datum, dokumentart, drucksachetyp, herausgeber)
            VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT DO NOTHING
        "#, fund.pdf_url, fund.id, fund.dokumentnummer, fund.datum, fund.dokumentart, fund.drucksachetyp, fund.herausgeber).execute(pool).await?;

    sqlx::query!(r#"
            INSERT INTO aktivitaeten(id, aktivitaetsart, typ, vorgangsbezug_anzahl, dokumentart, wahlperiode, datum, titel, fundstelle_id) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#, akt.id, akt.aktivitaetsart, akt.typ, akt.vorgangsbezug_anzahl, akt.dokumentart, akt.wahlperiode, akt.datum, akt.titel, fund.id).execute(pool).await?;

    for urheber in fund.urheber.iter() {
        sqlx::query!(r#"
                INSERT INTO fundstellen_urheber(fundstelle_id, urheber) 
                VALUES ($1, $2) 
            "#, fund.id, urheber).execute(pool).await?;
    }

    if let Some(vorgangsbezuege) = akt.vorgangsbezug {
        for v in vorgangsbezuege.iter() {
            sqlx::query!(r#"
                    INSERT INTO vorgangsbezuege(id, vorgangsposition, vorgangstyp, titel) 
                    VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING
                "#, v.id, v.vorgangsposition, v.vorgangstyp, v.titel).execute(pool).await?;
        }
    }

    Ok(())
}
