use chrono::NaiveDate;
use common::*;
use std::borrow::Borrow;
use std::fmt::Display;
use yew::prelude::*;

/// Performs validation on a stateful value using a validator function. This
/// will automatically set the error state.
pub fn validate<V, I, O, F, E>(
    value_state: UseStateHandle<V>,
    error_state: UseStateHandle<Option<String>>,
    validator: F,
) -> Option<O>
where
    F: FnOnce(&I) -> Result<O, E>,
    V: Borrow<I>,
    I: ?Sized,
    E: Display,
{
    match validator((*value_state).borrow()) {
        Ok(value) => {
            error_state.set(None);
            Some(value)
        }
        Err(err) => {
            error_state.set(Some(err.to_string()));
            None
        }
    }
}

/// Like `validate`, but accepts a value to be passed to the validator
/// function.
pub fn validate_with<V, I, O, F, E, T>(
    value_state: UseStateHandle<V>,
    error_state: UseStateHandle<Option<String>>,
    validator: F,
    with: T,
) -> Option<O>
where
    F: FnOnce(&I, T) -> Result<O, E>,
    V: Borrow<I>,
    I: ?Sized,
    E: Display,
{
    match validator((*value_state).borrow(), with) {
        Ok(value) => {
            error_state.set(None);
            Some(value)
        }
        Err(err) => {
            error_state.set(Some(err.to_string()));
            None
        }
    }
}

/// Performs validation on a variable number of stateful values and collapses
/// the resulting `Option`s into a single `Option`.
macro_rules! validate_all {
    // Generate the `validate` function call
    ( @validator_call $value_state:ident, $error_state:ident, $validator:ident ) => {
        $crate::validation::validate($value_state, $error_state, $validator)
    };

    // Generate the `validate_with` function call
    ( @validator_call $value_state:ident, $error_state:ident, $validator:ident with $with:expr ) => {
        $crate::validation::validate_with($value_state, $error_state, $validator, $with)
    };

    // Generate code to call all validators and collapse all `Option`s
    ( $( $value_state:ident, $error_state:ident, $validator:ident $( with $with:expr )? ; )* ) => {
        match ( $( $crate::validation::validate_all!( @validator_call $value_state, $error_state, $validator $( with $with )? ) ),* ) {
            #[allow(unused_parens)]
            ( $( Some($value_state) ),* ) => Some(( $( $value_state ),* )),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    };
}

pub(crate) use validate_all;

const STANDARD_NAME_MIN_LENGTH: usize = 1;
const STANDARD_NAME_MAX_LENGTH: usize = 255;
const STANDARD_DESCRIPTION_MAX_LENGTH: usize = 1023;
const SAVE_NAME_MIN_LENGTH: usize = STANDARD_NAME_MIN_LENGTH;
const SAVE_NAME_MAX_LENGTH: usize = STANDARD_NAME_MAX_LENGTH;
const SAVE_DESCRIPTION_MAX_LENGTH: usize = STANDARD_DESCRIPTION_MAX_LENGTH;
const SAVE_PASSWORD_MIN_LENGTH: usize = 8;
const SAVE_PASSWORD_MAX_LENGTH: usize = 255;
const ACCOUNT_NAME_MIN_LENGTH: usize = STANDARD_NAME_MIN_LENGTH;
const ACCOUNT_NAME_MAX_LENGTH: usize = STANDARD_NAME_MAX_LENGTH;
const ACCOUNT_DESCRIPTION_MAX_LENGTH: usize = STANDARD_DESCRIPTION_MAX_LENGTH;
const TRANSACTION_NAME_MIN_LENGTH: usize = STANDARD_NAME_MIN_LENGTH;
const TRANSACTION_NAME_MAX_LENGTH: usize = STANDARD_NAME_MAX_LENGTH;
const TRANSACTION_DESCRIPTION_MAX_LENGTH: usize = STANDARD_DESCRIPTION_MAX_LENGTH;

pub fn validate_save_name(name: &str) -> Result<String, String> {
    if name.len() < SAVE_NAME_MIN_LENGTH {
        Err(format!(
            "Save name must be at least {} characters long",
            SAVE_NAME_MIN_LENGTH
        ))
    } else if name.len() > SAVE_NAME_MAX_LENGTH {
        Err(format!(
            "Save name must be at most {} characters long",
            SAVE_NAME_MAX_LENGTH
        ))
    } else {
        Ok(name.to_owned())
    }
}

pub fn validate_save_description(description: &str) -> Result<String, String> {
    if description.len() > SAVE_DESCRIPTION_MAX_LENGTH {
        Err(format!(
            "Save description must be at most {} characters long",
            SAVE_DESCRIPTION_MAX_LENGTH
        ))
    } else {
        Ok(description.to_owned())
    }
}

pub fn validate_save_password(password: &str, confirm_password: &str) -> Result<String, String> {
    if password.len() < SAVE_PASSWORD_MIN_LENGTH {
        Err(format!(
            "Save password must be at least {} characters long",
            SAVE_PASSWORD_MIN_LENGTH
        ))
    } else if password.len() > SAVE_PASSWORD_MAX_LENGTH {
        Err(format!(
            "Save password must be at most {} characters long",
            SAVE_PASSWORD_MAX_LENGTH
        ))
    } else if password != confirm_password {
        Err("Passwords do not match".to_owned())
    } else {
        Ok(password.to_owned())
    }
}

pub fn validate_account_type(
    maybe_account_type: &Option<AccountType>,
) -> Result<AccountType, String> {
    match maybe_account_type {
        Some(account_type) => Ok(*account_type),
        None => Err("Please select an account type".to_owned()),
    }
}

pub fn validate_account_name(name: &str) -> Result<String, String> {
    if name.len() < ACCOUNT_NAME_MIN_LENGTH {
        Err(format!(
            "Account name must be at least {} characters long",
            ACCOUNT_NAME_MIN_LENGTH
        ))
    } else if name.len() > ACCOUNT_NAME_MAX_LENGTH {
        Err(format!(
            "Account name must be at most {} characters long",
            ACCOUNT_NAME_MAX_LENGTH
        ))
    } else {
        Ok(name.to_owned())
    }
}

pub fn validate_account_description(description: &str) -> Result<String, String> {
    if description.len() > ACCOUNT_DESCRIPTION_MAX_LENGTH {
        Err(format!(
            "Account description must be at most {} characters long",
            ACCOUNT_DESCRIPTION_MAX_LENGTH
        ))
    } else {
        Ok(description.to_owned())
    }
}

pub fn validate_transaction_name(name: &str) -> Result<String, String> {
    if name.len() < TRANSACTION_NAME_MIN_LENGTH {
        Err(format!(
            "Transaction name must be at least {} characters long",
            TRANSACTION_NAME_MIN_LENGTH
        ))
    } else if name.len() > TRANSACTION_NAME_MAX_LENGTH {
        Err(format!(
            "Transaction name must be at most {} characters long",
            TRANSACTION_NAME_MAX_LENGTH
        ))
    } else {
        Ok(name.to_owned())
    }
}

pub fn validate_transaction_description(description: &str) -> Result<String, String> {
    if description.len() > TRANSACTION_DESCRIPTION_MAX_LENGTH {
        Err(format!(
            "Transaction description must be at most {} characters long",
            TRANSACTION_DESCRIPTION_MAX_LENGTH
        ))
    } else {
        Ok(description.to_owned())
    }
}

pub fn validate_transaction_type(
    maybe_transaction_type: &Option<TransactionType>,
) -> Result<TransactionType, String> {
    match maybe_transaction_type {
        Some(transaction_type) => Ok(*transaction_type),
        None => Err("Please select a transaction type".to_owned()),
    }
}

pub fn validate_transaction_institution(
    maybe_institution: &Option<Institution>,
) -> Result<Institution, String> {
    match maybe_institution {
        Some(institution) => Ok(institution.clone()),
        None => Err("Please select a transaction institution".to_owned()),
    }
}

pub fn validate_transaction_date(maybe_date: &Option<NaiveDate>) -> Result<NaiveDate, String> {
    match maybe_date {
        Some(date) => Ok(*date),
        None => Err("Please select a transaction date".to_owned()),
    }
}

pub fn validate_transaction_category(
    maybe_category: &Option<Category>,
) -> Result<Category, String> {
    match maybe_category {
        Some(category) => Ok(category.clone()),
        None => Err("Please select a transaction category".to_owned()),
    }
}

pub fn validate_transaction_subcategory(
    maybe_subcategory: &Option<Subcategory>,
    maybe_category: &Option<Category>,
) -> Result<Option<Subcategory>, String> {
    match (maybe_category, maybe_subcategory) {
        (Some(category), Some(subcategory)) => {
            if subcategory.category_id == category.id {
                Ok(Some(subcategory.clone()))
            } else {
                Err("Invalid subcategory selected".to_owned())
            }
        }
        (None, Some(_)) => Err("Invalid subcategory selected".to_owned()),
        (_, None) => Ok(None),
    }
}
