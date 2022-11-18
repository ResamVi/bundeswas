use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Plenarprotokoll {
    pub id: String,
}

/// Allows for pretty printing JSON
impl core::fmt::Display for Plenarprotokoll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}


#[derive(Serialize, Deserialize)]
pub struct Vorgangsbezug {
    pub vorgangsposition: String,
    pub vorgangstyp: String,
    pub titel: String,
    pub id: String,
}
