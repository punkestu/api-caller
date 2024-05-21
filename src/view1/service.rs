use reqwest::header::HeaderName;
use serde_json::Value;
use std::{result, str::FromStr};

type Result<T> = result::Result<T, String>;

pub fn format_json(json: String) -> Result<String> {
    match serde_json::from_str::<Value>(json.as_str()) {
        Ok(j) => {
            let mut count = 1;
            if let Some(j) = j.as_array() {
                count = j.len();
            }
            match serde_json::to_string_pretty(&j) {
                Ok(json) => Ok(format!("count: {count}\n{json}")),
                Err(err) => Err(err.to_string()),
            }
        }
        Err(_) => Ok(json),
    }
}

pub fn request(method: &str, url: &str, headers: Vec<String>, body: String) -> Result<String> {
    let client = reqwest::blocking::Client::new();
    let mut request = client.request(
        reqwest::Method::from_str(method.to_uppercase().as_str())
            .map_err(|err| -> String { err.to_string() })?,
        url,
    );

    request = headers.into_iter().fold(request, |req, header| {
        let header_split: Vec<&str> = header.split(':').collect::<Vec<&str>>();
        if header_split.len() < 2 {
            return req;
        }
        let key = header_split[0];
        let value = header_split[1..].join(":");
        req.header(
            HeaderName::from_str(key.trim().replace(' ', "-").replace('\t', "--").as_str())
                .unwrap(),
            value,
        )
    });

    let request = request
        .body(body)
        .build()
        .map_err(|err| -> String { err.to_string() })?;
    match client.execute(request) {
        Ok(response) => {
            let content_type = response.headers().get("content-type");
            match content_type {
                Some(t) => {
                    let content_type = t.to_str().map_err(|err| -> String { err.to_string() })?;
                    if content_type.contains("application/json") {
                        let formated = format_json(
                            response
                                .text()
                                .map_err(|err| -> String { err.to_string() })?,
                        )?;
                        return Ok(formated);
                    }
                    Ok(response
                        .text()
                        .map_err(|err| -> String { err.to_string() })?)
                }
                None => Ok(response
                    .text()
                    .map_err(|err| -> String { err.to_string() })?),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}
