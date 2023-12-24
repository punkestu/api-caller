use std::str::FromStr;

use reqwest::Result;

pub fn request(method: &str, url: &str) -> Result<String> {
    let client = reqwest::blocking::Client::new();
    let request = client
        .request(
            reqwest::Method::from_str(method.to_uppercase().as_str()).unwrap(),
            url,
        )
        .build()?;
    match client.execute(request) {
        Ok(response) => response.text(),
        Err(err) => Err(err),
    }
}
