use core::fmt;
use core::iter;

use std::str::FromStr;
use reqwest::blocking::Client;
use reqwest::header;
use serde::{Deserialize, Serialize};

// Notes on structs:
// We require 'Serialize' for 'to_string_pretty' to pretty print to console
// 'Deserialize' for HTTP response body -> rust struct mapping

// TODO: Maybe Generics are possible here?
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")] // just for num_found -> numFound :S
struct Response {
    num_found: i32,
    documents: Vec<Aktivitaet>,
    cursor: String,
}


#[derive(Serialize, Deserialize)]
struct Aktivitaet {
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

impl fmt::Display for Aktivitaet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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


fn main() {
    let bundestag = BundestagsAPI::new();

    for i in bundestag.aktivitaeten().take(5) {
        println!("{}", i);
    };
}

// TODO: This should be a module I think?

/// Retrieve resources provided by the Deutscher Bundestag
/// what they coined "Dokumentations- und Informationssystem" (DIP).
///
/// We have permission for 25 parallel API calls at most.
struct BundestagsAPI { 
    // HTTP dispatcher
    client: Client,
}

impl BundestagsAPI {

    fn new() -> BundestagsAPI {
        // Bundestag requires API-Key
        // TODO: Request custom key at parlamentsdokumentation@bundestag.de
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_static("ApiKey GmEPb1B.bfqJLIhcGAsH9fTJevTglhFpCoZyAAAdhp")
        );

        // TODO: Handle error
        let client = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        BundestagsAPI { 
            client,
        }
    }

    /// Get a list of all Aktivitäten.
    // TODO: Magic values
    fn aktivitaeten(self) -> impl Iterator<Item = Aktivitaet> {
        Iter::new(self.client, String::from("https://search.dip.bundestag.de/api/v1/aktivitaet"))
    }
}

// Allows for consuming paginated results.
// DIP implements paginated results with a cursor parameter that needs to be appended to the next
// request.
//
// Retrieves pages and when all entries of the page are consumed dispatches another request for the
// next page.
// TODO: Generic implementation
struct Iter {
    client: Client,
    url: String,
    cursor: String,
    entries: Vec<Aktivitaet>,
}

impl Iter {
    fn new(client: Client, url: String) -> Iter {
        // TODO: Error handling
        let body = client.get(&url).send().unwrap().text().unwrap();
        let mut t: Response = serde_json::from_str(&body).unwrap();
        t.documents.reverse();

        Iter {
            client,
            url,
            cursor: String::from(""),
            entries: t.documents,
        }
    }
}

impl iter::Iterator for Iter {
    type Item = Aktivitaet;

    fn next(&mut self) -> Option<Aktivitaet> {
        // Refill when empty
        if self.entries.is_empty() {
            let url = format!("{}?cursor={}", self.url, self.cursor);

            // TODO: handle error
            let body = self.client.get(url).send().unwrap().text().unwrap();
            let mut t: Response = serde_json::from_str(&body).unwrap();
            t.documents.reverse();

            self.entries = t.documents;
        }

        self.entries.pop()
    }
}
