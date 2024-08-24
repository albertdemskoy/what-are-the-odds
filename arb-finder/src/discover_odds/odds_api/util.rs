use reqwest::header::HeaderMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApiKeyUsage {
    pub requests_used: i32,
    pub requests_remaining: i32,
}

fn get_typed_header(headers: &HeaderMap, header_name: &str) -> Option<i32> {
    let raw_val = match headers.get(header_name) {
        Some(x) => x,
        None => return None,
    };

    let string_val = raw_val.to_str().unwrap();
    return Some(string_val.parse::<i32>().unwrap());
}

pub fn get_key_usage_from_headers(headers: &HeaderMap) -> Option<ApiKeyUsage> {
    let requests_used_header_name = "x-requests-used";
    let requests_remaining_header_name = "x-requests-remaining";

    let reqs_used_val = match get_typed_header(headers, &requests_used_header_name) {
        Some(val) => val,
        None => return None,
    };

    let reqs_remaining_val = match get_typed_header(headers, &requests_remaining_header_name) {
        Some(val) => val,
        None => return None,
    };

    return Some(ApiKeyUsage {
        requests_used: reqs_used_val,
        requests_remaining: reqs_remaining_val,
    });
}
