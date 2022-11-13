use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Aktivitaet {
    id: String,
    aktivitaetsart: String,
    typ: String,
    vorgangsbezug_anzahl: i32,
    dokumentart: String,
    wahlperiode: i32,
    datum: String, // TODO: type ok?
    titel: String,

    fundstelle: Fundstelle,
    vorgangsbezug: Vec<Vorgangsbezug>,
}

/// Allows for pretty printing JSON
impl core::fmt::Display for Aktivitaet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}

#[derive(Serialize, Deserialize)]
struct Fundstelle {
    pdf_url: String,
    id: String,
    dokumentnummer: String,
    datum: String, // TODO: type
    dokumentart: String,
    drucksachetyp: String,
    herausgeber: String,
    urheber: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Vorgangsbezug {
    vorgangsposition: String,
    vorgangstyp: String,
    titel: String,
    id: String,
}
