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