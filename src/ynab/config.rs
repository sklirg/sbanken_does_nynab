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
        budget_id: match var("SBDNY_YNAB_BUDGET") {
            Ok(val) => val,
            Err(error) => {
                panic!("Failed to read ynab budget id from env: {}", error);
            }
        }
    };

    return config;
}
