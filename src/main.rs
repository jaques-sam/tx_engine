use std::path::PathBuf;
use tx_engine::{bank::BankError, transactions, transactions::ParseTxError};

fn main() -> Result<(), String> {
    let mut args = std::env::args().into_iter();

    args.next(); // skip the app name

    let transactions_abs_path = PathBuf::from(args.next().expect("No transaction CSV file given!"));

    let transactions = match transactions::parse_transactions(&transactions_abs_path) {
        Err(err) => match err.current_context() {
            ParseTxError::InvalidInput(msg) => return Err(format!("Invalid input: {msg}")),
            ParseTxError::Other => return Err("Internal error!".to_owned()),
        },
        Ok(transactions) => transactions,
    };

    let mut bank = tx_engine::bank::Bank::new();
    if let Err(err) = bank.handle_transactions(transactions) {
        match err.current_context() {
            BankError::InvalidInput => return Err("Invalid input!".to_owned()),
            BankError::Other => return Err("Internal error!".to_owned()),
        }
    }

    let mut writer = std::io::stdout().lock();
    if let Err(_) = bank.output_accounts_report(&mut writer) {
        return Err("Cannot export client report to CSV from input".to_owned());
    }

    Ok(())
}
