use std::env::{var};

use ynab::model::{YnabConfig};

pub fn get_config() -> YnabConfig {
    let config: YnabConfig = YnabConfig {
        access_token: match var("SBDNY_YNAB_TOKEN") {
            Ok(val) => val,
            Err(error) => {
                panic!("Failed to read ynab token from env: {}", error);
            }
        },
    };

    return config;
}
