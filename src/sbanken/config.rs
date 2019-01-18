use std::env::{var};

use sbanken::model::{SbankenConfig};

pub fn get_config() -> SbankenConfig {
    let config: SbankenConfig = SbankenConfig {
        username: match var("SBDNY_SBANKEN_USERNAME") {
            Ok(val) => val,
            Err(error) => {
                panic!("Failed to read sbanken username from env: {}", error);
            }
        },
        password: match var("SBDNY_SBANKEN_PASSWORD") {
            Ok(val) => val,
            Err(error) => {
                panic!("Failed to read sbanken password from env: {}", error);
            }
        },
        customer_id: match var("SBDNY_SBANKEN_CUSTOMER_ID") {
            Ok(val) => val,
            Err(error) => {
                panic!("Failed to read sbanken customer id from env: {}", error);
            }
        }
    };

    return config;
}
