use reqwest::header;
use helpers::http::{AuthenticationType, generate_auth_header};
use ynab::config::{get_config};

pub fn build_api_client() -> reqwest::Client {
    debug!("Building HTTP Client for authorized API requests");
    let config = get_config();

    let auth_header = generate_auth_header(
        config.access_token,
        AuthenticationType::Bearer);

    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, auth_header);

    return match reqwest::Client::builder()
        .default_headers(headers)
        .build() {
            Ok(client) => client,
            Err(error) => {
                error!("Building HTTP Client failed: {}", error);
                panic!("building client failed");
            },
        };;
}
