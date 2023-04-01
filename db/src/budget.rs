use crate::{Account, Timeframe, DB};
use chrono::NaiveDateTime;

/// An error when performing an budget operation.
#[derive(Debug, Clone, Copy)]
pub enum BudgetError {
    /// A budget already exists for the specified account.
    BudgetAlreadyExists,
}

/// A representation of an account budget in the database.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Budget {
    /// The ID of the account the budget is associated with.
    pub account_id: String,
    /// The note associated with the budget.
    pub note: Option<String>,
    /// The budget limit.
    pub total_limit: f64,
    /// The budget timeframe.
    pub timeframe: String,
    /// The time offset of the budget.
    pub timeframe_offset: NaiveDateTime,
    /// When the budget was created.
    pub created_at: NaiveDateTime,
}

impl Budget {
    /// Creates a new budget.
    pub async fn create(
        db: &DB,
        account: &Account,
        note: &str,
        limit: f64,
        timeframe: Timeframe,
        timeframe_offset: NaiveDateTime,
    ) -> Result<Self, BudgetError> {
        match Self::get(&db, &account).await {
            Some(_budget) => Err(BudgetError::BudgetAlreadyExists),
            None => {
                let timeframe_name = timeframe.to_internal_name();

                sqlx::query!("INSERT INTO budget (account_id, note, total_limit, timeframe, timeframe_offset) VALUES (?, ?, ?, ?, ?);", account.id, note, limit, timeframe_name, timeframe_offset).execute(&**db).await.unwrap();

                Ok(Self::get(&db, &account).await.unwrap())
            }
        }
    }

    /// Gets the budget for the specified account.
    pub async fn get(db: &DB, account: &Account) -> Option<Self> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM budget WHERE account_id = ?;",
            account.id
        )
        .fetch_optional(&**db)
        .await
        .unwrap()
    }

    /// Lists all budgets in the database.
    pub async fn list(db: &DB) -> Vec<Self> {
        sqlx::query_as!(Self, "SELECT * FROM budget ORDER BY created_at;")
            .fetch_all(&**db)
            .await
            .unwrap()
    }

    /// Gets the account the budget is associated with.
    pub async fn get_account(&self, db: &DB) -> Account {
        Account::get(&db, &self.account_id).await.unwrap()
    }

    /// Gets the timeframe.
    pub fn get_timeframe(&self) -> Timeframe {
        Timeframe::from_internal_name(&self.timeframe).unwrap()
    }

    /// Sets the associated account.
    pub async fn set_account(&mut self, db: &DB, account: &Account) {
        let old_account_id = self.account_id.clone();
        self.account_id = account.id.clone();

        sqlx::query!(
            "UPDATE budget SET account_id = ? WHERE account_id = ?;",
            self.account_id,
            old_account_id
        )
        .execute(&**db)
        .await
        .unwrap();
    }

    /// Sets the budget note.
    pub async fn set_note(&mut self, db: &DB, note: &str) {
        self.note = Some(note.to_owned());

        sqlx::query!(
            "UPDATE budget SET note = ? WHERE account_id = ?;",
            self.note,
            self.account_id
        )
        .execute(&**db)
        .await
        .unwrap();
    }

    /// Sets the budget limit.
    pub async fn set_limit(&mut self, db: &DB, limit: f64) {
        self.total_limit = limit;

        sqlx::query!(
            "UPDATE budget SET total_limit = ? WHERE account_id = ?;",
            self.total_limit,
            self.account_id
        )
        .execute(&**db)
        .await
        .unwrap();
    }

    /// Sets the timeframe.
    pub async fn set_timeframe(&mut self, db: &DB, timeframe: Timeframe) {
        self.timeframe = timeframe.to_internal_name();

        sqlx::query!(
            "UPDATE budget SET timeframe = ? WHERE account_id = ?;",
            self.timeframe,
            self.account_id
        )
        .execute(&**db)
        .await
        .unwrap();
    }

    /// Sets the timeframe offset.
    pub async fn set_timeframe_offset(&mut self, db: &DB, timeframe_offset: NaiveDateTime) {
        self.timeframe_offset = timeframe_offset;

        sqlx::query!(
            "UPDATE budget SET timeframe_offset = ? WHERE account_id = ?;",
            self.timeframe_offset,
            self.account_id
        )
        .execute(&**db)
        .await
        .unwrap();
    }

    /// Deletes the budget from the database.
    pub async fn delete(self, db: &DB) {
        sqlx::query!("DELETE FROM budget WHERE account_id = ?;", self.account_id)
            .execute(&**db)
            .await
            .unwrap();
    }
}

/// Budget tests.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AccountType, TestDB};

    #[tokio::test]
    async fn test_budget() {
        // Init
        let db = TestDB::new().await.unwrap();

        // Create
        let account1 = Account::create(&db, AccountType::Investment, "My investments", "").await;
        let account2 = Account::create(&db, AccountType::Property, "My property", "").await;
        let mut budget1 = Budget::create(
            &db,
            &account1,
            "My budget",
            123.45,
            Timeframe::Semiannually,
            NaiveDateTime::from_timestamp_millis(0).unwrap(),
        )
        .await
        .unwrap();
        Budget::create(
            &db,
            &account1,
            "My invalid budget",
            999.99,
            Timeframe::Biweekly,
            NaiveDateTime::from_timestamp_millis(1).unwrap(),
        )
        .await
        .unwrap_err();

        // Get
        let budget2 = Budget::get(&db, &account1).await.unwrap();
        assert_eq!(budget2, budget1);
        assert!(Budget::get(&db, &account2).await.is_none());

        // List
        let budgets = Budget::list(&db).await;
        assert_eq!(budgets.len(), 1);
        assert_eq!(budgets[0], budget1);

        // Get account
        let account3 = budget1.get_account(&db).await;
        assert_eq!(account3, account1);

        // Get timeframe
        assert_eq!(budget1.get_timeframe(), Timeframe::Semiannually);

        // Set account
        assert_eq!(budget1.account_id, account1.id);
        assert!(Budget::get(&db, &account1).await.is_some());
        assert!(Budget::get(&db, &account2).await.is_none());
        budget1.set_account(&db, &account2).await;
        assert_eq!(budget1.account_id, account2.id);
        assert!(Budget::get(&db, &account1).await.is_none());
        assert!(Budget::get(&db, &account2).await.is_some());
        let account4 = budget1.get_account(&db).await;
        assert_eq!(account4, account2);
        assert_ne!(account4, account1);

        // Set note
        budget1.set_note(&db, "New note").await;
        let budget3 = Budget::get(&db, &account2).await.unwrap();
        assert_eq!(budget3.note.as_ref().unwrap().as_str(), "New note");
        assert_eq!(budget3, budget1);

        // Set limit
        budget1.set_limit(&db, 234.56).await;
        let budget4 = Budget::get(&db, &account2).await.unwrap();
        assert_eq!(budget4.total_limit, 234.56);
        assert_eq!(budget4, budget1);

        // Set timeframe
        budget1.set_timeframe(&db, Timeframe::Quarterly).await;
        let budget5 = Budget::get(&db, &account2).await.unwrap();
        assert_eq!(budget5.get_timeframe(), Timeframe::Quarterly);
        assert_eq!(budget5, budget1);

        // Set timeframe offset
        budget1
            .set_timeframe_offset(&db, NaiveDateTime::from_timestamp_millis(1).unwrap())
            .await;
        let budget6 = Budget::get(&db, &account2).await.unwrap();
        assert_eq!(
            budget6.timeframe_offset,
            NaiveDateTime::from_timestamp_millis(1).unwrap()
        );
        assert_eq!(budget6, budget1);

        // Delete
        assert!(Budget::get(&db, &account2).await.is_some());
        budget1.delete(&db).await;
        assert!(Budget::get(&db, &account2).await.is_none());

        // Clean up
        db.delete().await.unwrap();
    }
}
