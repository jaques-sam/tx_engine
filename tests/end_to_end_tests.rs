#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use tx_engine::{bank::AccountReport, bank::Bank, transactions};

    #[test]
    fn test_a_client_got_a_dispute_on_a_failed_withdrawal() {
        let mut csv_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        csv_file.push("input_data");
        csv_file.push("withdrawal_disputed.csv");

        let mut bank = Bank::new();
        let actual_transactions = transactions::parse_transactions(&csv_file);

        bank.handle_transactions(actual_transactions.unwrap());

        let expected = vec![AccountReport::new(1, 0.0, 0.0, 0.0, false)];

        assert_eq!(bank.get_accounts_report(), expected);
    }

    #[test]
    fn test_a_client_got_a_resolve_on_a_failed_withdrawal() {
        let mut csv_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        csv_file.push("input_data");
        csv_file.push("withdrawal_resolved.csv");

        let mut bank = Bank::new();
        let actual_transactions = transactions::parse_transactions(&csv_file);

        bank.handle_transactions(actual_transactions.unwrap());

        let expected = vec![AccountReport::new(1, 0.0, 0.0, 0.0, false)];

        assert_eq!(bank.get_accounts_report(), expected);
    }

    #[test]
    fn test_a_client_got_a_chargeback_on_a_failed_withdrawal() {
        let mut csv_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        csv_file.push("input_data");
        csv_file.push("withdrawal_chargeback.csv");

        let mut bank = Bank::new();
        let actual_transactions = transactions::parse_transactions(&csv_file);

        bank.handle_transactions(actual_transactions.unwrap());

        let expected = vec![AccountReport::new(1, 0.0, 0.0, 0.0, false)];

        assert_eq!(bank.get_accounts_report(), expected);
    }

    #[test]
    fn test_a_client_got_a_dispute_on_a_deposit() {
        let mut csv_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        csv_file.push("input_data");
        csv_file.push("deposit_disputed.csv");

        let mut bank = Bank::new();
        let actual_transactions = transactions::parse_transactions(&csv_file);

        bank.handle_transactions(actual_transactions.unwrap());

        let expected = vec![AccountReport::new(1, 0.0, 4.0, 4.0, false)];

        assert_eq!(bank.get_accounts_report(), expected);
    }

    #[test]
    fn test_a_client_got_a_resolve_on_a_deposit() {
        let mut csv_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        csv_file.push("input_data");
        csv_file.push("deposit_resolved.csv");

        let mut bank = Bank::new();
        let actual_transactions = transactions::parse_transactions(&csv_file);

        bank.handle_transactions(actual_transactions.unwrap());

        let expected = vec![AccountReport::new(1, 4.0, 0.0, 4.0, false)];

        assert_eq!(bank.get_accounts_report(), expected);
    }

    #[test]
    fn test_a_client_got_a_chargeback_on_a_deposit() {
        let mut csv_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        csv_file.push("input_data");
        csv_file.push("deposit_chargeback.csv");

        let mut bank = Bank::new();
        let actual_transactions = transactions::parse_transactions(&csv_file);

        bank.handle_transactions(actual_transactions.unwrap());

        let expected = vec![AccountReport::new(1, 0.0, 0.0, 0.0, true)];

        assert_eq!(bank.get_accounts_report(), expected);
    }

    #[test]
    fn test_three_clients_have_their_correct_funds_after_handling_transactions() {
        let mut csv_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        csv_file.push("input_data");
        csv_file.push("multiple_client_transactions.csv");

        let mut bank = Bank::new();
        let actual_transactions = transactions::parse_transactions(&csv_file);

        bank.handle_transactions(actual_transactions.unwrap());

        let expected = vec![
            AccountReport::new(1, 1.0, 0.0, 1.0, true),
            AccountReport::new(2, 2.0, 0.0, 2.0, false),
            AccountReport::new(3, 5.0, 0.0, 5.0, false),
        ];

        assert_eq!(bank.get_accounts_report(), expected);
    }
}
