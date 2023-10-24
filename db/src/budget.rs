use crate::{Account, Timeframe, DB};
use backend_common::Result;
use chrono::NaiveDateTime;
use common::ExpectedCommandError as Error;

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
        db: &mut DB,
        account: &Account,
        note: &str,
        limit: f64,
        timeframe: Timeframe,
        timeframe_offset: NaiveDateTime,
    ) -> Result<Self> {
        match Self::get(db, account).await? {
            Some(_budget) => Err(Error::BudgetAlreadyExists)?,
            None => {
                let timeframe_name = timeframe.to_internal_name();

                sqlx::query!("INSERT INTO budget (account_id, note, total_limit, timeframe, timeframe_offset) VALUES (?, ?, ?, ?, ?);", account.id, note, limit, timeframe_name, timeframe_offset).execute(&mut **db).await?;

                Self::get(db, account).await.map(|x| x.unwrap())
            }
        }
    }

    /// Gets the budget for the specified account.
    pub async fn get(db: &mut DB, account: &Account) -> Result<Option<Self>> {
        Ok(sqlx::query_as!(
            Self,
            "SELECT * FROM budget WHERE account_id = ?;",
            account.id
        )
        .fetch_optional(&mut **db)
        .await?)
    }

    /// Lists all budgets in the database.
    pub async fn list(db: &mut DB) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM budget ORDER BY created_at;")
                .fetch_all(&mut **db)
                .await?,
        )
    }

    /// Gets the account the budget is associated with.
    pub async fn get_account(&self, db: &mut DB) -> Result<Account> {
        Account::get(db, &self.account_id).await.map(|x| x.unwrap())
    }

    /// Gets the timeframe.
    pub fn get_timeframe(&self) -> Timeframe {
        Timeframe::from_internal_name(&self.timeframe).unwrap()
    }

    /// Sets the associated account.
    pub async fn set_account(&mut self, db: &mut DB, account: &Account) -> Result<()> {
        let old_account_id = self.account_id.clone();
        self.account_id = account.id.clone();

        sqlx::query!(
            "UPDATE budget SET account_id = ? WHERE account_id = ?;",
            self.account_id,
            old_account_id
        )
        .execute(&mut **db)
        .await?;

        Ok(())
    }

    /// Sets the budget note.
    pub async fn set_note(&mut self, db: &mut DB, note: &str) -> Result<()> {
        self.note = Some(note.to_owned());

        sqlx::query!(
            "UPDATE budget SET note = ? WHERE account_id = ?;",
            self.note,
            self.account_id
        )
        .execute(&mut **db)
        .await?;

        Ok(())
    }

    /// Sets the budget limit.
    pub async fn set_limit(&mut self, db: &mut DB, limit: f64) -> Result<()> {
        self.total_limit = limit;

        sqlx::query!(
            "UPDATE budget SET total_limit = ? WHERE account_id = ?;",
            self.total_limit,
            self.account_id
        )
        .execute(&mut **db)
        .await?;

        Ok(())
    }

    /// Sets the timeframe.
    pub async fn set_timeframe(&mut self, db: &mut DB, timeframe: Timeframe) -> Result<()> {
        self.timeframe = timeframe.to_internal_name();

        sqlx::query!(
            "UPDATE budget SET timeframe = ? WHERE account_id = ?;",
            self.timeframe,
            self.account_id
        )
        .execute(&mut **db)
        .await?;

        Ok(())
    }

    /// Sets the timeframe offset.
    pub async fn set_timeframe_offset(
        &mut self,
        db: &mut DB,
        timeframe_offset: NaiveDateTime,
    ) -> Result<()> {
        self.timeframe_offset = timeframe_offset;

        sqlx::query!(
            "UPDATE budget SET timeframe_offset = ? WHERE account_id = ?;",
            self.timeframe_offset,
            self.account_id
        )
        .execute(&mut **db)
        .await?;

        Ok(())
    }

    /// Deletes the budget from the database.
    pub async fn delete(self, db: &mut DB) -> Result<()> {
        sqlx::query!("DELETE FROM budget WHERE account_id = ?;", self.account_id)
            .execute(&mut **db)
            .await?;

        Ok(())
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
        let mut db = TestDB::new().await.unwrap();

        // Create
        let account1 = Account::create(&mut db, AccountType::Investment, "My investments", "")
            .await
            .unwrap();
        let account2 = Account::create(&mut db, AccountType::Property, "My property", "")
            .await
            .unwrap();
        let mut budget1 = Budget::create(
            &mut db,
            &account1,
            "My budget",
            123.45,
            Timeframe::Semiannually,
            NaiveDateTime::from_timestamp_millis(0).unwrap(),
        )
        .await
        .unwrap();
        Budget::create(
            &mut db,
            &account1,
            "My invalid budget",
            999.99,
            Timeframe::Biweekly,
            NaiveDateTime::from_timestamp_millis(1).unwrap(),
        )
        .await
        .unwrap_err();

        // Get
        let budget2 = Budget::get(&mut db, &account1).await.unwrap().unwrap();
        assert_eq!(budget2, budget1);
        assert!(Budget::get(&mut db, &account2).await.unwrap().is_none());

        // List
        let budgets = Budget::list(&mut db).await.unwrap();
        assert_eq!(budgets.len(), 1);
        assert_eq!(budgets[0], budget1);

        // Get account
        let account3 = budget1.get_account(&mut db).await.unwrap();
        assert_eq!(account3, account1);

        // Get timeframe
        assert_eq!(budget1.get_timeframe(), Timeframe::Semiannually);

        // Set account
        assert_eq!(budget1.account_id, account1.id);
        assert!(Budget::get(&mut db, &account1).await.unwrap().is_some());
        assert!(Budget::get(&mut db, &account2).await.unwrap().is_none());
        budget1.set_account(&mut db, &account2).await.unwrap();
        assert_eq!(budget1.account_id, account2.id);
        assert!(Budget::get(&mut db, &account1).await.unwrap().is_none());
        assert!(Budget::get(&mut db, &account2).await.unwrap().is_some());
        let account4 = budget1.get_account(&mut db).await.unwrap();
        assert_eq!(account4, account2);
        assert_ne!(account4, account1);

        // Set note
        budget1.set_note(&mut db, "New note").await.unwrap();
        let budget3 = Budget::get(&mut db, &account2).await.unwrap().unwrap();
        assert_eq!(budget3.note.as_ref().unwrap().as_str(), "New note");
        assert_eq!(budget3, budget1);

        // Set limit
        budget1.set_limit(&mut db, 234.56).await.unwrap();
        let budget4 = Budget::get(&mut db, &account2).await.unwrap().unwrap();
        assert_eq!(budget4.total_limit, 234.56);
        assert_eq!(budget4, budget1);

        // Set timeframe
        budget1
            .set_timeframe(&mut db, Timeframe::Quarterly)
            .await
            .unwrap();
        let budget5 = Budget::get(&mut db, &account2).await.unwrap().unwrap();
        assert_eq!(budget5.get_timeframe(), Timeframe::Quarterly);
        assert_eq!(budget5, budget1);

        // Set timeframe offset
        budget1
            .set_timeframe_offset(&mut db, NaiveDateTime::from_timestamp_millis(1).unwrap())
            .await
            .unwrap();
        let budget6 = Budget::get(&mut db, &account2).await.unwrap().unwrap();
        assert_eq!(
            budget6.timeframe_offset,
            NaiveDateTime::from_timestamp_millis(1).unwrap()
        );
        assert_eq!(budget6, budget1);

        // Delete
        assert!(Budget::get(&mut db, &account2).await.unwrap().is_some());
        budget1.delete(&mut db).await.unwrap();
        assert!(Budget::get(&mut db, &account2).await.unwrap().is_none());

        // Clean up
        db.delete().await.unwrap();
    }
}
