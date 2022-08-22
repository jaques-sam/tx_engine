#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use tx_engine::transactions::{Kind, Transaction};

    #[test]
    fn test_basic_transactions_are_correctly_read_from_csv() {
        let mut csv_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        csv_file.push("input_data");
        csv_file.push("basic_transactions.csv");

        let expected_transactions = vec![
            Transaction::new(Kind::Deposit, 1, 1, Some(1.0)),
            Transaction::new(Kind::Deposit, 2, 2, Some(2.0)),
            Transaction::new(Kind::Deposit, 1, 3, Some(2.0)),
            Transaction::new(Kind::Withdrawal, 1, 4, Some(1.5)),
            Transaction::new(Kind::Withdrawal, 2, 5, Some(3.0)),
        ];

        let actual_transactions = tx_engine::transactions::read_transactions(&csv_file);
        assert_eq!(actual_transactions, expected_transactions);
    }

    #[test]
    fn test_transactions_with_optional_field_are_correctly_read_from_csv() {
        let mut csv_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        csv_file.push("input_data");
        csv_file.push("all_transactions_types.csv");

        let expected_transactions = vec![
            Transaction::new(Kind::Deposit, 1, 1, Some(1.0)),
            Transaction::new(Kind::Withdrawal, 1, 2, Some(3.0)),
            Transaction::new(Kind::Dispute, 1, 2, None),
            Transaction::new(Kind::Resolve, 1, 2, None),
            Transaction::new(Kind::Chargeback, 1, 2, None),
        ];

        let actual_transactions = tx_engine::transactions::read_transactions(&csv_file);
        assert_eq!(actual_transactions, expected_transactions);
    }
}
