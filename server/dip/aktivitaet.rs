use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Aktivitaet {
    pub id: String,
    // "Kleine Anfrage"     - Frage eines Parlamentariers an die Exekutive (wird nicht beraten)
    // "Antrag"             - Etwas worüber abgestimmt und vom Parlament beschlossen werden kann.
    // "Frage"              - Frage eines Parlamentariers
    // "Antwort"            - Antwort von der Exekutive
    // "Berichterstattung"  - Beschlussempfehlung und Berichte von Ausschüssen über Anträge
    // Entschließungsantrag
    // Gesetzentwurf
    // Änderungsantrag
    // Schriftliche Erklärung gem. § 31 Geschäftsordnung BT
    // Rede (zu Protokoll gegeben)
    // Zwischenfrage
    // -- Ab hier großer Sprung (2000 vs 940)
    // Zusatzfrage
    // Kurzintervention
    // Erwiderung
    // Zur Geschäftsordnung BT
    // "Große Anfrage" - Wird schriftlich beantwortet und im Bundestag debattiert
    // Einleitende Ausführungen und Beantwortung
    // Erklärung zum Vermittlungsverfahren (§91 GO-BT, §10 GO-VermA)
    // Erklärung zur Aussprache gem. § 30 Geschäftsordnung BT
    // Mündliche Erklärung gem. § 31 Geschäftsordnung BT
    // Persönliche Erklärung gem. § 32 Geschäftsordnung BT
    // Wortbeitrag
    pub aktivitaetsart: String,
    pub typ: String,
    pub vorgangsbezug_anzahl: i32,

    // Aktivitäten sind von ihrer Dokumentart entweder "Drucksache" oder "Plenarprotokoll"
    pub dokumentart: String,
    pub wahlperiode: i32,
    pub datum: String, // TODO: type ok?
    pub titel: String,

    pub fundstelle: Fundstelle,
    pub vorgangsbezug: Option<Vec<Vorgangsbezug>>,
}

/// Allows for pretty printing JSON
impl core::fmt::Display for Aktivitaet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}


/// Fundstelle gibt Auskunft wo sich mehr zu einer Aktivität/Plenarprotokoll 
/// finden lässt
#[derive(Serialize, Deserialize)]
pub struct Fundstelle {
    pub pdf_url: String,
    pub id: String,
    pub dokumentnummer: String,
    pub datum: String, // TODO: type
    pub dokumentart: String,
    pub drucksachetyp: Option<String>,
    pub herausgeber: String,
    pub urheber: Vec<String>,
}

/// Vorgangsbezüge hangen an Aktivitäten/Plenarprotokolle
#[derive(Serialize, Deserialize)]
pub struct Vorgangsbezug {
    pub vorgangsposition: String,
    pub vorgangstyp: String,
    pub titel: String,
    pub id: String,
}
