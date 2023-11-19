use backend_common::Result;
use common::ExpectedCommandError as Error;

/// A representation of a transaction type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransactionType {
    /// A credit transaction.
    Credit,
    /// A debit transaction.
    Debit,
}

impl TransactionType {
    /// Gets the transaction type from its internal name in the database.
    pub fn from_internal_name(transaction_type: &str) -> Result<Self> {
        match transaction_type {
            "CREDIT" => Ok(Self::Credit),
            "DEBIT" => Ok(Self::Debit),
            _ => Err(Error::InvalidTransactionType)?,
        }
    }

    /// Gets the internal name of the transaction type.
    pub fn to_internal_name(&self) -> String {
        match self {
            Self::Credit => "CREDIT",
            Self::Debit => "DEBIT",
        }
        .to_owned()
    }

    /// Gets the human-readable string representation of the transaction type.
    pub fn as_str(&self) -> String {
        match self {
            Self::Credit => "Credit",
            Self::Debit => "Debit",
        }
        .to_owned()
    }
}

/// Transaction type tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_type() {
        // Parse transaction types
        let credit_transaction = TransactionType::from_internal_name("CREDIT").unwrap();
        assert_eq!(credit_transaction, TransactionType::Credit);
        let debit_transaction = TransactionType::from_internal_name("DEBIT").unwrap();
        assert_eq!(debit_transaction, TransactionType::Debit);
        TransactionType::from_internal_name("TRANSACTION").unwrap_err();
        TransactionType::from_internal_name("INVALID_TRANSACTION").unwrap_err();

        // Get transaction type string names
        assert_eq!(&credit_transaction.to_internal_name(), "CREDIT");
        assert_eq!(&debit_transaction.to_internal_name(), "DEBIT");

        // Get transaction type human-readable names
        assert_eq!(&credit_transaction.as_str(), "Credit");
        assert_eq!(&debit_transaction.as_str(), "Debit");
    }
}
