use reqwest::header;
use crate::helpers::http::{AuthenticationType, base64_encode_uname_pw, generate_auth_header, generate_header_value};
use crate::sbanken::config::{get_config};

pub fn build_authorization_http_client() -> reqwest::blocking::Client {
    debug!("Building HTTP Client for authorization");
    
    let config = get_config();
    let auth_header = generate_auth_header(
        base64_encode_uname_pw(config.username, config.password),
        AuthenticationType::Basic);

    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, auth_header);

    return match reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build() {
            Ok(client) => client,
            Err(error) => {
                error!("Building HTTP Client for authorization failed: {}", error);
                panic!("building client failed");
            },
        };
}

pub fn build_api_client(credentials: String) -> reqwest::blocking::Client {
    debug!("Building HTTP Client for authorized API requests");
    let config = get_config();

    let auth_header = generate_auth_header(
        credentials,
        AuthenticationType::Bearer);

    let customer_id_header = generate_header_value(&config.customer_id);

    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, auth_header);
    headers.insert("customerId", customer_id_header);


    return match reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build() {
            Ok(client) => client,
            Err(error) => {
                error!("Building HTTP Client failed: {}", error);
                panic!("building client failed");
            },
        };
}
