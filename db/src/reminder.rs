use crate::{new_id, Account, DBImpl, Timeframe};
use backend_common::Result;
use chrono::NaiveDateTime;

/// A representation of a reminder in the database.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reminder {
    /// The reminder's identifier.
    pub id: String,
    /// The ID of the account the reminder is associated with.
    pub account_id: String,
    /// The note associated with the reminder.
    pub note: Option<String>,
    /// The reminder timeframe.
    pub timeframe: String,
    /// The time offset of the reminder.
    pub timeframe_offset: NaiveDateTime,
    /// When the reminder was created.
    pub created_at: NaiveDateTime,
}

impl Reminder {
    /// Creates a new reminder.
    pub async fn create(
        db: &mut DBImpl,
        account: &Account,
        note: &str,
        timeframe: Timeframe,
        timeframe_offset: NaiveDateTime,
    ) -> Result<Self> {
        let id = new_id();
        let timeframe_name = timeframe.to_internal_name();

        sqlx::query!(
            "INSERT INTO reminder (id, account_id, note, timeframe, timeframe_offset) VALUES (?, ?, ?, ?, ?);",
            id,
            account.id,
            note,
            timeframe_name,
            timeframe_offset
        )
        .execute(&mut *db)
        .await?;

        Self::get(db, &id).await.map(|x| x.unwrap())
    }

    /// Gets a reminder from the database.
    pub async fn get(db: &mut DBImpl, id: &str) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM reminder WHERE id = ?;", id)
                .fetch_optional(&mut *db)
                .await?,
        )
    }

    /// Lists all reminders in the database.
    pub async fn list(db: &mut DBImpl) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM reminder ORDER BY created_at;")
                .fetch_all(&mut *db)
                .await?,
        )
    }

    /// Gets the account the reminder is associated with.
    pub async fn get_account(&self, db: &mut DBImpl) -> Result<Account> {
        Account::get(db, &self.account_id).await.map(|x| x.unwrap())
    }

    /// Gets the timeframe.
    pub fn get_timeframe(&self) -> Timeframe {
        Timeframe::from_internal_name(&self.timeframe).unwrap()
    }

    /// Sets the associated account.
    pub async fn set_account(&mut self, db: &mut DBImpl, account: &Account) -> Result<()> {
        self.account_id = account.id.clone();

        sqlx::query!(
            "UPDATE reminder SET account_id = ? WHERE id = ?;",
            self.account_id,
            self.id
        )
        .execute(&mut *db)
        .await?;

        Ok(())
    }

    /// Sets the reminder note.
    pub async fn set_note(&mut self, db: &mut DBImpl, note: &str) -> Result<()> {
        self.note = Some(note.to_owned());

        sqlx::query!(
            "UPDATE reminder SET note = ? WHERE id = ?;",
            self.note,
            self.id
        )
        .execute(&mut *db)
        .await?;

        Ok(())
    }

    /// Sets the timeframe.
    pub async fn set_timeframe(&mut self, db: &mut DBImpl, timeframe: Timeframe) -> Result<()> {
        self.timeframe = timeframe.to_internal_name();

        sqlx::query!(
            "UPDATE reminder SET timeframe = ? WHERE id = ?;",
            self.timeframe,
            self.id
        )
        .execute(&mut *db)
        .await?;

        Ok(())
    }

    /// Sets the timeframe offset.
    pub async fn set_timeframe_offset(
        &mut self,
        db: &mut DBImpl,
        timeframe_offset: NaiveDateTime,
    ) -> Result<()> {
        self.timeframe_offset = timeframe_offset;

        sqlx::query!(
            "UPDATE reminder SET timeframe_offset = ? WHERE id = ?;",
            self.timeframe_offset,
            self.id
        )
        .execute(&mut *db)
        .await?;

        Ok(())
    }

    /// Deletes the reminder from the database.
    pub async fn delete(self, db: &mut DBImpl) -> Result<()> {
        sqlx::query!("DELETE FROM reminder WHERE id = ?;", self.id)
            .execute(&mut *db)
            .await?;

        Ok(())
    }
}

/// Reminder tests.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AccountType, TestDB};

    #[tokio::test]
    async fn test_reminder() {
        // Init
        let mut db = TestDB::new().await.unwrap();

        // Create
        let account1 = Account::create(&mut db, AccountType::BankAccount, "My bank account", "")
            .await
            .unwrap();
        let mut reminder1 = Reminder::create(
            &mut db,
            &account1,
            "My reminder",
            Timeframe::Monthly,
            NaiveDateTime::from_timestamp_millis(0).unwrap(),
        )
        .await
        .unwrap();
        let reminder2 = Reminder::create(
            &mut db,
            &account1,
            "My other reminder",
            Timeframe::Weekly,
            NaiveDateTime::from_timestamp_millis(0).unwrap(),
        )
        .await
        .unwrap();

        // Get
        let reminder3 = Reminder::get(&mut db, &reminder1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(reminder3, reminder1);
        let reminder4 = Reminder::get(&mut db, &reminder2.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(reminder4, reminder2);
        assert!(Reminder::get(&mut db, "").await.unwrap().is_none());

        // List
        let reminders = Reminder::list(&mut db).await.unwrap();
        assert_eq!(reminders.len(), 2);
        let reminder5 = reminders.iter().find(|x| x.id == reminder1.id).unwrap();
        assert_eq!(reminder5, &reminder1);
        let reminder6 = reminders.iter().find(|x| x.id == reminder2.id).unwrap();
        assert_eq!(reminder6, &reminder2);

        // Get account
        let account2 = reminder1.get_account(&mut db).await.unwrap();
        assert_eq!(account2, account1);
        let account3 = reminder2.get_account(&mut db).await.unwrap();
        assert_eq!(account3, account1);

        // Get timeframe
        assert_eq!(reminder1.get_timeframe(), Timeframe::Monthly);
        assert_eq!(reminder2.get_timeframe(), Timeframe::Weekly);

        // Set account
        let account4 = Account::create(&mut db, AccountType::CreditCard, "My other account", "")
            .await
            .unwrap();
        reminder1.set_account(&mut db, &account4).await.unwrap();
        let account5 = reminder1.get_account(&mut db).await.unwrap();
        assert_eq!(account5, account4);
        assert_ne!(account5, account1);
        let account6 = reminder2.get_account(&mut db).await.unwrap();
        assert_eq!(account6, account1);
        assert_ne!(account6, account4);

        // Set note
        reminder1.set_note(&mut db, "New note").await.unwrap();
        let reminder7 = Reminder::get(&mut db, &reminder1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(reminder7.note.as_ref().unwrap().as_str(), "New note");
        assert_eq!(reminder7, reminder1);

        // Set timeframe
        reminder1
            .set_timeframe(&mut db, Timeframe::Quarterly)
            .await
            .unwrap();
        let reminder8 = Reminder::get(&mut db, &reminder1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(reminder8.get_timeframe(), Timeframe::Quarterly);
        assert_eq!(reminder8, reminder1);

        // Set timeframe offset
        reminder1
            .set_timeframe_offset(&mut db, NaiveDateTime::from_timestamp_millis(1).unwrap())
            .await
            .unwrap();
        let reminder9 = Reminder::get(&mut db, &reminder1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            reminder9.timeframe_offset,
            NaiveDateTime::from_timestamp_millis(1).unwrap()
        );
        assert_eq!(reminder9, reminder1);

        // Delete
        let reminder_id1 = reminder1.id.clone();
        assert!(Reminder::get(&mut db, &reminder_id1)
            .await
            .unwrap()
            .is_some());
        reminder1.delete(&mut db).await.unwrap();
        assert!(Reminder::get(&mut db, &reminder_id1)
            .await
            .unwrap()
            .is_none());
        let reminder_id2 = reminder2.id.clone();
        assert!(Reminder::get(&mut db, &reminder_id2)
            .await
            .unwrap()
            .is_some());
        reminder2.delete(&mut db).await.unwrap();
        assert!(Reminder::get(&mut db, &reminder_id2)
            .await
            .unwrap()
            .is_none());

        // Clean up
        db.delete().await.unwrap();
    }
}
