use serde::{Serialize, Deserialize};

/// Aktivitäten sind...
///
/// aktivitaetsart:
///     "Kleine Anfrage"     - Frage eines Parlamentariers an die Exekutive (wird nicht beraten)
///     "Antrag"             - Etwas worüber abgestimmt und vom Parlament beschlossen werden kann.
///     "Frage"              - Frage eines Parlamentariers
///     "Antwort"            - Antwort von der Exekutive
///     "Berichterstattung"  - Beschlussempfehlung und Berichte von Ausschüssen über Anträge
///     Entschließungsantrag
///     Gesetzentwurf
///     Änderungsantrag
///     Schriftliche Erklärung gem. § 31 Geschäftsordnung BT
///     Rede (zu Protokoll gegeben)
///     Zwischenfrage
///     -- Ab hier großer Sprung (2000 vs 940)
///     Zusatzfrage
///     Kurzintervention
///     Erwiderung
///     Zur Geschäftsordnung BT
///     "Große Anfrage" - Wird schriftlich beantwortet und im Bundestag debattiert
///     Einleitende Ausführungen und Beantwortung
///     Erklärung zum Vermittlungsverfahren (§91 GO-BT, §10 GO-VermA)
///     Erklärung zur Aussprache gem. § 30 Geschäftsordnung BT
///     Mündliche Erklärung gem. § 31 Geschäftsordnung BT
///     Persönliche Erklärung gem. § 32 Geschäftsordnung BT
///     Wortbeitrag
///
#[derive(Serialize, Deserialize)]
pub struct Aktivitaet {
    pub id: String,
    pub datum: String, // TODO: type ok?
    pub titel: String,

    /// dokumentart: entweder "Drucksache" oder "Plenarprotokoll"
    pub dokumentart: String,
    pub wahlperiode: i32,
    pub vorgangsbezug_anzahl: i32,

    pub aktivitaetsart: String,
    pub typ: String,

    pub fundstelle: Fundstelle,
    pub vorgangsbezug: Option<Vec<Vorgangsbezug>>,
}

/// ............
#[derive(Serialize, Deserialize)]
pub struct Plenarprotokoll {
    pub id: String,
    pub datum: String, // TODO: type ok?
    pub titel: String,

    /// dokumentart: entweder "Drucksache" oder "Plenarprotokoll"
    pub dokumentart: String,
    pub wahlperiode: i32,
    pub vorgangsbezug_anzahl: i32,

    pub dokumentnummer: String,
    pub herausgeber: String,

    pub fundstelle: Fundstelle,
    pub vorgangsbezug: Option<Vec<Vorgangsbezug>>,
}


/// ............
/// Es gibt Vorgänge die sehr kurz sind:
/// {
///     "id": "292386",
///     "vorgangstyp": "Unterrichtung durch das Europäische Parlament",
///     "typ": "Vorgang",
///     "wahlperiode": 20,
///     "initiative": [
///         "Bundestag"
///     ],
///     "datum": "2022-11-11",
///     "titel": "Entschließung des Europäischen Parlaments vom 15. September 2022\r\nMenschenrechtsverletzungen im Zusammenhang mit der Deportation ukrainischer Zivilisten nach Russland und der Zwangsadoption ukrainischer Kinder in Russland (2022/2825(RSP)) \r\nEP P9_TA(2022)0320"
/// }
#[derive(Serialize, Deserialize)]
pub struct Vorgang {
    pub id: String,
    pub datum: String, // TODO: type ok?
    pub titel: String,

    // TOOMUCH pub wahlperiode: i32,
    pub vorgangstyp: String,

    // immer "Vorgang"
    pub typ: String,
    pub initiative: Option<Vec<String>>,

    // TOOMUCH pub deskriptor: Option<Vec<Deskriptor>>,
    pub beratungsstand: Option<String>,
    pub sachgebiet: Option<Vec<String>>,

    // TODO vorgang_verlinkung
    // TODO gesta: String,
    // TODO abstract: String,
    // TODO zustimmungsbeduerftigkeit: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct Deskriptor {
    pub fundstelle: bool,
    pub name: String,
    pub typ: String,
}

/// Fundstelle gibt Auskunft wo sich mehr zu einer Aktivität/Plenarprotokoll 
/// finden lässt
#[derive(Serialize, Deserialize)]
pub struct Fundstelle {
    pub id: String,
    pub dokumentart: String,        /// dokumentart: entweder "Drucksache" oder "Plenarprotokoll"
    pub datum: String,              // TODO: type

    pub dokumentnummer: String,
    pub herausgeber: String,

    pub pdf_url: String,
    pub drucksachetyp: Option<String>,
    pub urheber: Vec<String>,
}

/// Vorgangsbezüge hangen an Aktivitäten/Plenarprotokolle
#[derive(Serialize, Deserialize)]
pub struct Vorgangsbezug {
    pub id: String,
    pub titel: String,
    pub vorgangstyp: String,

    /// Findet man wenn eingebettet in Aktivitäten aber nicht in Plenarprotokollen
    pub vorgangsposition: Option<String>,
}

// TODO: Duplication
/// Allows for pretty printing JSON
impl core::fmt::Display for Aktivitaet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}

/// Allows for pretty printing JSON
impl core::fmt::Display for Plenarprotokoll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}
///
/// Allows for pretty printing JSON
impl core::fmt::Display for Vorgang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}
