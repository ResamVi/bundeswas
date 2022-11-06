use core::fmt;
use core::iter;

use std::str::FromStr;
use reqwest::blocking::Client;
use reqwest::header;
use serde::{Deserialize, Serialize};

// Notes on structs:
// We require 'Serialize' for 'to_string_pretty' to pretty print to console
// 'Deserialize' for HTTP response body -> rust struct mapping

// TODO: Maybe Generics are possible here? I've tried but man is it HARD
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

/// Allows for pretty printing JSON
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

    for i in bundestag.aktivitaeten().take(101) {
        println!("{}", i.id);
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
        PaginatedResource::new(self.client, "https://search.dip.bundestag.de/api/v1/aktivitaet")
    }
}

// Allows for consuming resources that are paginated and must 
// be retrieved in multiple GET requests ("Folgeanfragen").
//
// DIP specifies paginated results via a "cursor" 
// parameter that needs to be appended to the next request.
//
// TODO: Generic implementation
struct PaginatedResource {
    // Static
    client: Client,
    url: String,

    // Is mutated while we iterate
    cursor: String,
    entries: Vec<Aktivitaet>,
}

impl PaginatedResource {

    /// A PaginatedResource requires a http client to dispatch more GET requests
    /// when the current page has been exhausted.
    fn new(client: Client, url: &str) -> PaginatedResource {
        PaginatedResource {
            client,
            url: url.to_string(),
            cursor: String::new(),
            entries: Vec::new(),
        }
    }
}

impl iter::Iterator for PaginatedResource {
    type Item = Aktivitaet;

    fn next(&mut self) -> Option<Aktivitaet> {
        // TODO: Stop when at the end (cursor doesn't change)

        // Refill when empty
        if self.entries.is_empty() {
            let mut url = self.url.clone();

            if !self.cursor.is_empty() {
                url.push_str("?cursor=");
                url.push_str(self.cursor.as_str());
            }

            // TODO: handle error
            let raw_response = self.client.get(url).send().unwrap().text().unwrap();
            let mut response: Response = serde_json::from_str(&raw_response).unwrap();
            response.documents.reverse();

            // TODO: check if cursor didnt change (signifies no more resources)

            self.cursor = response.cursor;
            self.entries = response.documents;
        }

        self.entries.pop()
    }
}
