// REWORK to do !!!
// Rework the Methods

// !Description
/*
    The Request string for openweathermap is build up.
    the update period is set to a fixed time because the purpose is to keep this project as a no cost project
    and to save the performance of the raspberry pi pico.

*/
// use moduals
extern crate serde_json;

//use http::StatusCode;

pub struct Request {
    version: f64,
    unit: str,
    location: u64,
    lang: str,
    api: str,
}

// Request data from the api
fn request_data(request: &Request) -> String {
    let url = String::from("http://api.openweathermap.org/data/");
    // Add Api Version
    url.push_str("{}", request.version.to_string());
    // Add location
    url.push_str("/weather?id={}", request.location.to_string());
    // Add unit
    url.push_str("&units={}", request.unit);
    // Add lang
    url.push_str("/weather?id={}", request.lang);
    // api key
    url.push_str("/weather?id={}", request.api);

    return url;
}

// VVVVVVVVVVVVVVVVVVVVVVVVVVVVV delete after finished VVVVVVVVVVVVVVVVVVVVVVVVVVVVV
//-------------------------------------------------------------------------------------
/*

Copy from https://crates.io/crates/openweathermap


extern crate reqwest;       // => not for embedded
extern crate serde_json;

use futures::executor;
use http::StatusCode;
use regex::Regex;           // => not for embedded
use std::sync::mpsc;        // => not for embedded
use std::thread;
  // => not for embedded
use std::time::Duration;

mod api;
pub use api::*;

#[cfg(test)]
mod tests;
pub fn init(location: &str, units: &str, lang: &str, api_key: &str, poll_mins: u64) -> Receiver {
    // generate correct request URL depending on city is id or name
    let url = match location.parse::<u64>().is_ok() {
        true => format!(
            "http://api.openweathermap.org/data/2.5/weather?id={}&units={}&lang={}&appid={}",
            location, units, lang, api_key
        ),
        false => {
            let re = Regex::new(r"(-?\d+\.\d+)\s*,\s*(-?\d+\.\d+)").unwrap();
            match re.captures(&location) {
                Some(caps) => format!(
                    "http://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&units={}&lang={}&appid={}",
                    caps.get(1).unwrap().as_str(),
                    caps.get(2).unwrap().as_str(),
                    units,
                    lang,
                    api_key
                ),
                None => format!(
                    "http://api.openweathermap.org/data/2.5/weather?q={}&units={}&lang={}&appid={}",
                    location, units, lang, api_key
                ),
            }
        }
    };
    // fork thread that continuously fetches weather updates every <poll_mins> minutes
    let period = Duration::from_secs(60 * poll_mins);
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(Err(LOADING.to_string())).unwrap_or(());
        loop {
            match reqwest::blocking::get(&url) {
                Ok(response) => match response.status() {
                    StatusCode::OK => match serde_json::from_str(&response.text().unwrap()) {
                        Ok(w) => {
                            tx.send(Ok(w)).unwrap_or(());
                            if period == Duration::new(0, 0) {
                                break;
                            }
                            thread::sleep(period);
                        }
                        Err(e) => tx.send(Err(e.to_string())).unwrap_or(()),
                    },
                    _ => tx.send(Err(response.status().to_string())).unwrap_or(()),
                },
                Err(_e) => (),
            }
        }
    });
    // return receiver that provides the updated weather as json string
    return rx;
}



pub fn update(receiver: &Receiver) -> Option<Result<CurrentWeather, String>> {
    match receiver.try_recv() {
        Ok(response) => Some(response),
        Err(_e) => None,
    }
}



pub async fn weather(
    location: &str,
    units: &str,
    lang: &str,
    api_key: &str,
) -> Result<CurrentWeather, String> {
    let r = init(location, units, lang, api_key, 0);
    loop {
        match update(&r) {
            Some(response) => match response {
                Ok(current) => return Ok(current),
                Err(e) => {
                    if e != LOADING {
                        return Err(e);
                    }
                }
            },
            None => (),
        }
    }
}

/// synchronous functions
pub mod blocking {
    use super::*;





    pub fn weather(
        location: &str,
        units: &str,
        lang: &str,
        api_key: &str,
    ) -> Result<CurrentWeather, String> {
        // wait for result
        executor::block_on(super::weather(location, units, lang, api_key))
    }
}



*/
