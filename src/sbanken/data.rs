use sbanken::model::{Account, SBankenAccountResponse, SBankenTransactionsResponse, Transaction};

pub fn accounts_response_to_account(resp: String) -> Vec<Account> {
    if resp.len() == 0 {
        return Vec::new();
    }

    trace!("Transforming account response {}", resp);
    let data: SBankenAccountResponse = match serde_json::from_str(&resp) {
        Ok(x) => x,
        Err(error) => {
            panic!("Something went wrong while destructuring account response: {}", error);
        }
    };

    return data.items;
}

pub fn transactions_response_to_transactions(resp: String) -> Vec<Transaction> {
    if resp.len() == 0 {
        return Vec::new();
    }

    trace!("Transforming transaction response {}", resp);
    let data: SBankenTransactionsResponse = match serde_json::from_str(&resp) {
        Ok(x) => x,
        Err(error) => {
            panic!("Something went wrong while destructuring transactions response: {}", error);
        }
    };

    return data.items;
}