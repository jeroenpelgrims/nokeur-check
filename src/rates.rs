use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Response {
    rates: HashMap<String, f64>,
}

pub fn get_nokeur() -> f64 {
    let app_id = std::env::var("APP_ID").expect("APP_ID must be set.");
    let url = format!(
        "https://openexchangerates.org/api/latest.json?app_id={}",
        app_id
    );
    let rates = reqwest::blocking::get(url)
        .expect("Error doing http request")
        .json::<Response>()
        .expect("Error parsing JSON")
        .rates;
    let eur = rates["EUR"];
    let nok = rates["NOK"];
    nok / eur
}
