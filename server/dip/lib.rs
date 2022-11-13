/// This crate implements calls to the DIP (Dokumentations- und Informationsystem für
/// Parlamentsmaterialien)
///
/// See their documentation [here](https://dip.bundestag.de/documents/informationsblatt_zur_dip_api.pdf)
use std::str::FromStr;
use reqwest::blocking::Client;
use reqwest::header;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

mod aktivitaet;

// Notes on structs:
// We require 'Serialize' for 'to_string_pretty' to pretty print to console
// 'Deserialize' for HTTP response body -> rust struct mapping

// TODO: Maybe Generics are possible here? I've tried but man is it HARD
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")] // just for num_found -> numFound ...
struct Response<T> {
    num_found: i32,
    documents: Vec<T>,
    cursor: String,
}


// TODO: This should be a module I think?

/// Retrieve resources provided by the Deutscher Bundestag
/// what they coined "Dokumentations- und Informationssystem" (DIP).
///
/// We have permission for 25 parallel API calls at most.
pub struct DIP { 
    // HTTP dispatcher
    client: Client,
}

pub fn new() -> DIP {
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

    DIP { 
        client,
    }
}

impl DIP {
    /// Get a list of all Aktivitäten.
    // TODO: Magic values
    pub fn aktivitaeten(self) -> impl Iterator<Item = aktivitaet::Aktivitaet> {
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
struct PaginatedResource<T> {
    // Static
    client: Client,
    url: String,

    // Is mutated while we iterate
    cursor: String,
    entries: Vec<T>,
    response: String,
}

impl<T> PaginatedResource<T> {

    /// A PaginatedResource requires a http client to dispatch more GET requests
    /// when the current page has been exhausted.
    fn new(client: Client, url: &str) -> PaginatedResource<T> {
        PaginatedResource {
            client,
            url: url.to_string(),
            cursor: String::new(),
            entries: Vec::new(),
            response: String::new(),
        }
    }
}

impl<T: DeserializeOwned> core::iter::Iterator for PaginatedResource<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Stop when at the end (cursor doesn't change)

        // Refill when empty
        if self.entries.is_empty() {
            let mut url = self.url.clone();

            if !self.cursor.is_empty() {
                url.push_str("?cursor=");
                url.push_str(self.cursor.as_str());
            }

            // TODO: handle error
            self.response = self.client.get(url).send().unwrap().text().unwrap();
            let mut response: Response<Self::Item> = serde_json::from_str(&self.response).unwrap();
            response.documents.reverse();

            // TODO: check if cursor didnt change (signifies no more resources)

            self.cursor = response.cursor;
            self.entries = response.documents;
        }

        self.entries.pop()
    }
}
