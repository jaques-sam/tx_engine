use std::path::PathBuf;
use tx_engine::transactions;

fn main() -> Result<(), String> {
    let mut args = std::env::args().into_iter();

    args.next(); // skip the app name

    let transactions_abs_path = PathBuf::from(args.next().expect("No transaction CSV file given!"));

    let transactions = transactions::parse_transactions(&transactions_abs_path);

    let mut bank = tx_engine::bank::Bank::new();
    bank.handle_transactions(transactions.unwrap());

    let mut writer = std::io::stdout().lock();
    bank.output_accounts_report(&mut writer);

    Ok(())
}
