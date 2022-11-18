use std::collections::HashMap;

fn main() -> Result<(), ureq::Error> {
    // let mut headers = header::HeaderMap::new();
    // headers.insert(
    //     header::AUTHORIZATION,
    //     header::HeaderValue::from_static("ApiKey GmEPb1B.bfqJLIhcGAsH9fTJevTglhFpCoZyAAAdhp")
    // );

    // let url = "https://search.dip.bundestag.de/api/v1/aktivitaet";
    // let resp = client.get(url).send().await?.text().await?;

    let body: String = ureq::get("https://search.dip.bundestag.de/api/v1/aktivitaet")
            .set("Authorization", "ApiKey GmEPb1B.bfqJLIhcGAsH9fTJevTglhFpCoZyAAAdhp")
            .call()?
            .into_string()?;
    // Ok(())
    println!("{}", body);
    Ok(())
}
