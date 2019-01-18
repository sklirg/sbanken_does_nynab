use ynab::model::{AccountBaseResponse, Account, BudgetBaseResponse, Budget};

pub fn budgets_response_to_budgets(resp: String) -> Vec<Budget> {
    if resp.len() == 0 {
        return Vec::new();
    }

    trace!("Transforming budgets response {}", resp);
    let data: BudgetBaseResponse = match serde_json::from_str(&resp) {
        Ok(x) => x,
        Err(error) => {
            panic!("Something went wrong while destructuring budgets response: {}", error);
        }
    };

    return data.data.budgets;
}

pub fn accounts_response_to_accounts(resp: String) -> Vec<Account> {
    if resp.len() == 0 {
        return Vec::new();
    }

    trace!("Transforming accounts response {}", resp);
    let data: AccountBaseResponse = match serde_json::from_str(&resp) {
        Ok(x) => x,
        Err(error) => {
            panic!("Something went wrong while destructuring budgets response: {}", error);
        }
    };

    return data.data.accounts;
}
