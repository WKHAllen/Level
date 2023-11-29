use crate::ExpectedCommandError as Error;
use crate::SelectOptions;
use serde::{Deserialize, Serialize};

/// A representation of an account type.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, SelectOptions,
)]
pub enum AccountType {
    /// A bank account.
    BankAccount,
    /// A retirement account.
    RetirementAccount,
    /// A credit card.
    CreditCard,
    /// A property asset.
    Property,
    /// A liability.
    Liability,
    /// An investment.
    Investment,
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.as_str())
    }
}

impl AccountType {
    /// Gets the account type from its internal name in the database.
    pub fn from_internal_name(account_type: &str) -> Result<Self, Error> {
        match account_type {
            "BANK_ACCOUNT" => Ok(Self::BankAccount),
            "RETIREMENT_ACCOUNT" => Ok(Self::RetirementAccount),
            "CREDIT_CARD" => Ok(Self::CreditCard),
            "PROPERTY" => Ok(Self::Property),
            "LIABILITY" => Ok(Self::Liability),
            "INVESTMENT" => Ok(Self::Investment),
            _ => Err(Error::InvalidAccountType)?,
        }
    }

    /// Gets the internal name of the account type.
    pub fn to_internal_name(&self) -> String {
        match self {
            Self::BankAccount => "BANK_ACCOUNT",
            Self::RetirementAccount => "RETIREMENT_ACCOUNT",
            Self::CreditCard => "CREDIT_CARD",
            Self::Property => "PROPERTY",
            Self::Liability => "LIABILITY",
            Self::Investment => "INVESTMENT",
        }
        .to_owned()
    }

    /// Gets the human-readable string representation of the account type.
    pub fn as_str(&self) -> String {
        match self {
            Self::BankAccount => "Bank account",
            Self::RetirementAccount => "Retirement account",
            Self::CreditCard => "Credit card",
            Self::Property => "Property",
            Self::Liability => "Liability",
            Self::Investment => "Investment",
        }
        .to_owned()
    }
}

/// Account type tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_type() {
        // Parse account types
        let bank_account = AccountType::from_internal_name("BANK_ACCOUNT").unwrap();
        assert_eq!(bank_account, AccountType::BankAccount);
        let retirement_account = AccountType::from_internal_name("RETIREMENT_ACCOUNT").unwrap();
        assert_eq!(retirement_account, AccountType::RetirementAccount);
        let credit_card = AccountType::from_internal_name("CREDIT_CARD").unwrap();
        assert_eq!(credit_card, AccountType::CreditCard);
        let property = AccountType::from_internal_name("PROPERTY").unwrap();
        assert_eq!(property, AccountType::Property);
        let liability = AccountType::from_internal_name("LIABILITY").unwrap();
        assert_eq!(liability, AccountType::Liability);
        let investment = AccountType::from_internal_name("INVESTMENT").unwrap();
        assert_eq!(investment, AccountType::Investment);
        AccountType::from_internal_name("ACCOUNT").unwrap_err();
        AccountType::from_internal_name("INVALID_ACCOUNT").unwrap_err();

        // Get account type string names
        assert_eq!(&bank_account.to_internal_name(), "BANK_ACCOUNT");
        assert_eq!(&retirement_account.to_internal_name(), "RETIREMENT_ACCOUNT");
        assert_eq!(&credit_card.to_internal_name(), "CREDIT_CARD");
        assert_eq!(&property.to_internal_name(), "PROPERTY");
        assert_eq!(&liability.to_internal_name(), "LIABILITY");
        assert_eq!(&investment.to_internal_name(), "INVESTMENT");

        // Get account type human-readable names
        assert_eq!(&bank_account.as_str(), "Bank account");
        assert_eq!(&retirement_account.as_str(), "Retirement account");
        assert_eq!(&credit_card.as_str(), "Credit card");
        assert_eq!(&property.as_str(), "Property");
        assert_eq!(&liability.as_str(), "Liability");
        assert_eq!(&investment.as_str(), "Investment");
    }
}
