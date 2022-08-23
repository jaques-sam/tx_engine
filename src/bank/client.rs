use std::{error::Error, fmt};

pub type Amount = f64;

fn round_at_4_dec(x: Amount) -> Amount {
    (x * 10000.0).round() / 10000.0
}

#[derive(Debug, PartialEq)]
pub enum AccountError {
    InsufficientFunds(String),
}

impl Error for AccountError {}

impl fmt::Display for AccountError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Wrong account operation")
    }
}

#[derive(Default)]
pub struct Account {
    available_funds: Amount,
    held_funds: Amount,
    locked: bool,
}

impl Account {
    pub fn new() -> Account {
        Account::default()
    }

    pub fn get_available_funds(&self) -> Amount {
        round_at_4_dec(self.available_funds)
    }

    pub fn get_held_funds(&self) -> Amount {
        round_at_4_dec(self.held_funds)
    }

    pub fn get_total_funds(&self) -> Amount {
        round_at_4_dec(self.available_funds + self.held_funds)
    }

    pub fn is_locked(&self) -> bool {
        self.locked
    }

    /// A credit to the client's asset account with an amount
    ///
    /// # Examples
    /// ```
    /// use tx_engine::client::Account;
    /// let mut account = Account::new();
    /// account.deposit(2.0);
    /// assert_eq!(account.get_available_funds(), 2.0);
    /// ```
    pub fn deposit(&mut self, amount: Amount) {
        self.available_funds += amount;
    }

    /// A debit to client's asset account with an amount
    ///
    /// # Examples
    /// ```
    /// use tx_engine::client::Account;
    /// let mut account = Account::new();
    /// account.deposit(2.0);
    /// account.withdrawal(1.0);
    /// assert_eq!(account.get_available_funds(), 1.0);
    /// ```
    pub fn withdrawal(&mut self, amount: Amount) -> Result<(), AccountError> {
        if (self.available_funds - amount) < 0.0 {
            return Err(AccountError::InsufficientFunds(
                "Insufficient funds to withdraw".to_owned(),
            ));
        }

        self.available_funds -= amount;
        Ok(())
    }

    /// An amount under dispute which becomes held
    pub fn dispute(&mut self, amount: Amount) {
        self.available_funds -= amount;
        self.held_funds += amount;
    }

    /// A resolution to a dispute which releases the held funds
    pub fn resolve(&mut self, amount: Amount) {
        self.available_funds += amount;
        self.held_funds -= amount;
    }

    /// A resolution to a dispute that locks the account
    pub fn chargeback(&mut self, amount: Amount) {
        self.held_funds -= amount;
        self.locked = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accounts_are_initialized_with_zero_funds_and_are_unlocked() {
        let account = Account::new();
        assert_eq!(account.get_available_funds(), 0.0);
        assert_eq!(account.get_held_funds(), 0.0);
        assert_eq!(account.get_total_funds(), 0.0);
        assert_eq!(account.is_locked(), false);
    }

    #[test]
    fn test_account_deposits_increases_funds() {
        let mut account = Account::new();
        account.deposit(1.0);
        account.deposit(2.0);
        account.deposit(3.0);

        assert_eq!(account.get_available_funds(), 6.0);
        assert_eq!(account.get_total_funds(), 6.0);
        assert_eq!(account.get_held_funds(), 0.0);
    }

    #[test]
    fn test_account_withdrawal_decreases_funds() -> Result<(), AccountError> {
        let mut account = Account::new();
        account.deposit(6.0);

        account.withdrawal(3.0)?;
        account.withdrawal(2.0)?;
        assert_eq!(account.get_available_funds(), 1.0);
        assert_eq!(account.get_total_funds(), 1.0);
        assert_eq!(account.get_held_funds(), 0.0);
        Ok(())
    }

    #[test]
    fn test_account_dispute_descreases_available_funds_and_increases_held_funds() {
        let mut account = Account::new();
        account.deposit(6.0);

        account.dispute(4.0);
        assert_eq!(account.get_available_funds(), 2.0);
        assert_eq!(account.get_held_funds(), 4.0);
        assert_eq!(account.get_total_funds(), 6.0);
    }

    #[test]
    fn test_account_resolve_increases_available_and_held_funds() {
        let mut account = Account::new();
        account.deposit(6.0);

        account.resolve(3.0);
        assert_eq!(account.get_available_funds(), 9.0);
        assert_eq!(account.get_held_funds(), -3.0);
        assert_eq!(account.get_total_funds(), 6.0);
    }

    #[test]
    fn test_account_resolve_reverts_a_dispute() {
        let mut account = Account::new();
        account.deposit(6.0);

        account.dispute(4.0);

        account.resolve(4.0);
        assert_eq!(account.get_available_funds(), 6.0);
        assert_eq!(account.get_held_funds(), 0.0);
        assert_eq!(account.get_total_funds(), 6.0);
    }

    #[test]
    fn test_account_chargeback_locks_account_and_decreases_total_and_held_funds() {
        let mut account = Account::new();
        account.deposit(6.0);

        account.dispute(4.0);

        account.chargeback(4.0);
        assert_eq!(account.is_locked(), true);
        assert_eq!(account.get_available_funds(), 2.0);
        assert_eq!(account.get_held_funds(), 0.0);
        assert_eq!(account.get_total_funds(), 2.0);
    }

    #[test]
    fn test_account_withdrawal_fails_on_insufficient_funds() {
        let mut account = Account::new();
        assert!(matches!(
            account.withdrawal(1.0).unwrap_err(),
            AccountError::InsufficientFunds(_)
        ));
    }

    #[test]
    fn test_precision_of_funds_are_4_digits_past_the_decimal_with_closest_integer_rounding() {
        let mut account = Account::new();
        account.deposit(4.0001);
        account.deposit(4.00005);

        assert_eq!(account.get_available_funds(), 8.0002);
    }

    #[test]
    fn test_two_values_half_the_precision_results_in_the_smallest_precision_number() {
        let mut account = Account::new();
        account.deposit(0.00005);
        account.deposit(0.00005);

        assert_eq!(account.get_available_funds(), 0.0001);
    }
}
