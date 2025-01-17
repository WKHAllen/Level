use crate::{DBAccount, DBImpl};
use async_trait::async_trait;
use backend_common::Result;
use chrono::NaiveDateTime;
use common::{ExpectedCommandError as Error, *};

/// The database implementation of the budget model.
#[async_trait]
pub trait DBBudget: Sized {
    /// Creates a new budget.
    async fn create(
        db: &mut DBImpl,
        account: &Account,
        note: &str,
        limit: f64,
        timeframe: Timeframe,
        timeframe_offset: NaiveDateTime,
    ) -> Result<Self>;

    /// Gets the budget for the specified account.
    async fn get(db: &mut DBImpl, account: &Account) -> Result<Option<Self>>;

    /// Lists all budgets in the database.
    async fn list(db: &mut DBImpl) -> Result<Vec<Self>>;

    /// Gets the account the budget is associated with.
    async fn get_account(&self, db: &mut DBImpl) -> Result<Account>;

    /// Sets the associated account.
    async fn set_account(&mut self, db: &mut DBImpl, account: &Account) -> Result<()>;

    /// Sets the budget note.
    async fn set_note(&mut self, db: &mut DBImpl, note: &str) -> Result<()>;

    /// Sets the budget limit.
    async fn set_limit(&mut self, db: &mut DBImpl, limit: f64) -> Result<()>;

    /// Sets the timeframe.
    async fn set_timeframe(&mut self, db: &mut DBImpl, timeframe: Timeframe) -> Result<()>;

    /// Sets the timeframe offset.
    async fn set_timeframe_offset(
        &mut self,
        db: &mut DBImpl,
        timeframe_offset: NaiveDateTime,
    ) -> Result<()>;

    /// Deletes the budget from the database.
    async fn delete(self, db: &mut DBImpl) -> Result<()>;
}

#[async_trait]
impl DBBudget for Budget {
    async fn create(
        db: &mut DBImpl,
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

                sqlx::query!("INSERT INTO budget (account_id, note, total_limit, timeframe, timeframe_offset) VALUES (?, ?, ?, ?, ?);", account.id, note, limit, timeframe_name, timeframe_offset).execute(&mut *db).await?;

                Self::get(db, account).await.map(|x| x.unwrap())
            }
        }
    }

    async fn get(db: &mut DBImpl, account: &Account) -> Result<Option<Self>> {
        Ok(sqlx::query_as!(
            Self,
            "SELECT * FROM budget WHERE account_id = ?;",
            account.id
        )
        .fetch_optional(&mut *db)
        .await?)
    }

    async fn list(db: &mut DBImpl) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM budget ORDER BY created_at;")
                .fetch_all(&mut *db)
                .await?,
        )
    }

    async fn get_account(&self, db: &mut DBImpl) -> Result<Account> {
        Account::get(db, &self.account_id).await.map(|x| x.unwrap())
    }

    async fn set_account(&mut self, db: &mut DBImpl, account: &Account) -> Result<()> {
        let old_account_id = self.account_id.clone();
        self.account_id = account.id.clone();

        sqlx::query!(
            "UPDATE budget SET account_id = ? WHERE account_id = ?;",
            self.account_id,
            old_account_id
        )
        .execute(&mut *db)
        .await?;

        Ok(())
    }

    async fn set_note(&mut self, db: &mut DBImpl, note: &str) -> Result<()> {
        self.note = Some(note.to_owned());

        sqlx::query!(
            "UPDATE budget SET note = ? WHERE account_id = ?;",
            self.note,
            self.account_id
        )
        .execute(&mut *db)
        .await?;

        Ok(())
    }

    async fn set_limit(&mut self, db: &mut DBImpl, limit: f64) -> Result<()> {
        self.total_limit = limit;

        sqlx::query!(
            "UPDATE budget SET total_limit = ? WHERE account_id = ?;",
            self.total_limit,
            self.account_id
        )
        .execute(&mut *db)
        .await?;

        Ok(())
    }

    async fn set_timeframe(&mut self, db: &mut DBImpl, timeframe: Timeframe) -> Result<()> {
        self.timeframe = timeframe.to_internal_name();

        sqlx::query!(
            "UPDATE budget SET timeframe = ? WHERE account_id = ?;",
            self.timeframe,
            self.account_id
        )
        .execute(&mut *db)
        .await?;

        Ok(())
    }

    async fn set_timeframe_offset(
        &mut self,
        db: &mut DBImpl,
        timeframe_offset: NaiveDateTime,
    ) -> Result<()> {
        self.timeframe_offset = timeframe_offset;

        sqlx::query!(
            "UPDATE budget SET timeframe_offset = ? WHERE account_id = ?;",
            self.timeframe_offset,
            self.account_id
        )
        .execute(&mut *db)
        .await?;

        Ok(())
    }

    async fn delete(self, db: &mut DBImpl) -> Result<()> {
        sqlx::query!("DELETE FROM budget WHERE account_id = ?;", self.account_id)
            .execute(&mut *db)
            .await?;

        Ok(())
    }
}

/// Budget tests.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::TestDB;

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
