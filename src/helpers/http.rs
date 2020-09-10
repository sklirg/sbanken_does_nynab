extern crate base64;
extern crate urlencoding;

use self::base64::{encode};
use self::urlencoding::{encode as urlencode};
use reqwest::header;

pub enum AuthenticationType {
    Basic,
    Bearer,
}

pub fn base64_encode_uname_pw(username: String, password: String) -> String {
    return encode(&format!("{}:{}", urlencode(&username), urlencode(&password)));
}

pub fn generate_auth_header(credentials: String, auth_type: AuthenticationType) -> header::HeaderValue {
    let auth_header = match auth_type {
        AuthenticationType::Basic => format!("Basic {}", credentials),
        AuthenticationType::Bearer => format!("Bearer {}", credentials),
    };

    return generate_header_value(auth_header);
}

pub fn generate_header_value(value: String) -> header::HeaderValue {
    return match header::HeaderValue::from_str(&value) {
        Ok(val) => val,
        Err(error) => {
            panic!(format!("Failed to generate HTTP header: {}", error));
        }
    };
}
