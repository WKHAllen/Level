use crate::{new_id, Account, Category, DBImpl, Institution, Subcategory, TransactionType};
use backend_common::Result;
use chrono::{NaiveDate, NaiveDateTime, Utc};
use common::ExpectedCommandError as Error;

/// A representation of an account transaction in the database.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct AccountTransaction {
    /// The account transaction's identifier.
    pub id: String,
    /// The ID of the account which the transaction is associated with.
    pub account_id: String,
    /// The name of the account.
    pub name: String,
    /// A description of the account.
    pub description: Option<String>,
    /// The monetary amount of the transaction.
    pub amount: f64,
    /// The type of transaction.
    pub transaction_type: String,
    /// The ID of the institution which the transaction is associated with.
    pub institution_id: String,
    /// The date of the transaction.
    pub transaction_date: NaiveDateTime,
    /// The ID of the category in which the transaction exists.
    pub category_id: String,
    /// The ID of the subcategory in which the transaction exists.
    pub subcategory_id: Option<String>,
    /// Whether the transaction has been reconciled.
    pub reconciled: bool,
    /// When the transaction was created.
    pub created_at: NaiveDateTime,
    /// When the transaction was last edited.
    pub edited_at: Option<NaiveDateTime>,
    /// When the transaction was last reconciled.
    pub reconciled_at: Option<NaiveDateTime>,
}

impl AccountTransaction {
    /// Creates a new account transaction. This can fail if the category/subcategory combination is invalid.
    pub async fn create(
        db: &mut DBImpl,
        account: &mut Account,
        name: &str,
        description: &str,
        amount: f64,
        transaction_type: TransactionType,
        institution: &Institution,
        date: NaiveDate,
        category: &Category,
        subcategory: Option<&Subcategory>,
    ) -> Result<Self> {
        if let Some(given_subcategory) = subcategory {
            if given_subcategory.category_id != category.id {
                Err(Error::InvalidSubcategory)?;
            }
        }

        let id = new_id();
        let transaction_type_name = transaction_type.to_internal_name();
        let transaction_date = date.and_hms_milli_opt(12, 0, 0, 0).unwrap();
        let subcategory_id = subcategory.map(|x| x.id.as_str());

        sqlx::query!(
            "INSERT INTO account_transaction (id, account_id, name, description, amount, transaction_type, institution_id, transaction_date, category_id, subcategory_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?);",
            id,
            account.id,
            name,
            description,
            amount,
            transaction_type_name,
            institution.id,
            transaction_date,
            category.id,
            subcategory_id
        )
        .execute(&mut *db)
        .await?;

        account.mark_edited(db).await?;

        Self::get(db, &id).await.map(|x| x.unwrap())
    }

    /// Gets an account transaction from the database.
    pub async fn get(db: &mut DBImpl, id: &str) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM account_transaction WHERE id = ?;", id)
                .fetch_optional(&mut *db)
                .await?,
        )
    }

    /// Lists all account transactions in the database.
    pub async fn list(db: &mut DBImpl) -> Result<Vec<Self>> {
        Ok(sqlx::query_as!(
            Self,
            "SELECT * FROM account_transaction ORDER BY transaction_date, created_at;"
        )
        .fetch_all(&mut *db)
        .await?)
    }

    /// Lists all account transactions within a given account.
    pub async fn list_within(db: &mut DBImpl, account: &Account) -> Result<Vec<Self>> {
        Ok(sqlx::query_as!(
            Self,
            "SELECT * FROM account_transaction WHERE account_id = ? ORDER BY transaction_date, created_at;",
            account.id
        )
        .fetch_all(&mut *db)
        .await?)
    }

    /// Gets the account the transaction is associated with.
    pub async fn get_account(&self, db: &mut DBImpl) -> Result<Account> {
        Account::get(db, &self.account_id).await.map(|x| x.unwrap())
    }

    /// Gets the type of the transaction.
    pub fn get_transaction_type(&self) -> TransactionType {
        TransactionType::from_internal_name(&self.transaction_type).unwrap()
    }

    /// Gets the institution which the transaction is associated with.
    pub async fn get_institution(&self, db: &mut DBImpl) -> Result<Institution> {
        Institution::get(db, &self.institution_id)
            .await
            .map(|x| x.unwrap())
    }

    /// Gets the date the transaction took place.
    pub fn get_date(&self) -> NaiveDate {
        self.transaction_date.date()
    }

    /// Gets the category in which the transaction exists.
    pub async fn get_category(&self, db: &mut DBImpl) -> Result<Category> {
        Category::get(db, &self.category_id)
            .await
            .map(|x| x.unwrap())
    }

    /// Gets the subcategory in which the transaction exists.
    pub async fn get_subcategory(&self, db: &mut DBImpl) -> Result<Option<Subcategory>> {
        match &self.subcategory_id {
            Some(subcategory_id) => Subcategory::get(db, subcategory_id)
                .await
                .map(|x| Some(x.unwrap())),
            None => Ok(None),
        }
    }

    /// Marks the transaction as edited.
    pub async fn mark_edited(&mut self, db: &mut DBImpl) -> Result<()> {
        self.edited_at = Some(Utc::now().naive_utc());

        sqlx::query!(
            "UPDATE account_transaction SET edited_at = ? WHERE id = ?;",
            self.edited_at,
            self.id
        )
        .execute(&mut *db)
        .await?;

        self.get_account(db).await?.mark_edited(db).await?;

        Ok(())
    }

    /// Marks the transaction as reconciled.
    pub async fn mark_reconciled(&mut self, db: &mut DBImpl) -> Result<()> {
        self.reconciled_at = Some(Utc::now().naive_utc());

        sqlx::query!(
            "UPDATE account_transaction SET reconciled_at = ? WHERE id = ?;",
            self.reconciled_at,
            self.id
        )
        .execute(&mut *db)
        .await?;

        Ok(())
    }

    /// Sets the account the transaction is associated with.
    pub async fn set_account(&mut self, db: &mut DBImpl, account: &Account) -> Result<()> {
        self.account_id = account.id.clone();

        sqlx::query!(
            "UPDATE account_transaction SET account_id = ? WHERE id = ?;",
            self.account_id,
            self.id
        )
        .execute(&mut *db)
        .await?;

        self.mark_edited(db).await?;

        Ok(())
    }

    /// Sets the transaction name.
    pub async fn set_name(&mut self, db: &mut DBImpl, name: &str) -> Result<()> {
        self.name = name.to_owned();

        sqlx::query!(
            "UPDATE account_transaction SET name = ? WHERE id = ?;",
            self.name,
            self.id
        )
        .execute(&mut *db)
        .await?;

        self.mark_edited(db).await?;

        Ok(())
    }

    /// Sets the transaction description.
    pub async fn set_description(&mut self, db: &mut DBImpl, description: &str) -> Result<()> {
        self.description = Some(description.to_owned());

        sqlx::query!(
            "UPDATE account_transaction SET description = ? WHERE id = ?;",
            self.description,
            self.id
        )
        .execute(&mut *db)
        .await?;

        self.mark_edited(db).await?;

        Ok(())
    }

    /// Sets the transaction amount.
    pub async fn set_amount(&mut self, db: &mut DBImpl, amount: f64) -> Result<()> {
        self.amount = amount;

        sqlx::query!(
            "UPDATE account_transaction SET amount = ? WHERE id = ?;",
            self.amount,
            self.id
        )
        .execute(&mut *db)
        .await?;

        self.mark_edited(db).await?;

        Ok(())
    }

    /// Sets the date of the transaction.
    pub async fn set_date(&mut self, db: &mut DBImpl, date: NaiveDate) -> Result<()> {
        self.transaction_date = date.and_hms_milli_opt(12, 0, 0, 0).unwrap();

        sqlx::query!(
            "UPDATE account_transaction SET transaction_date = ? WHERE id = ?;",
            self.transaction_date,
            self.id
        )
        .execute(&mut *db)
        .await?;

        self.mark_edited(db).await?;

        Ok(())
    }

    /// Sets the transaction's category. This invalidates the subcategory, setting it to None.
    pub async fn set_category(&mut self, db: &mut DBImpl, category: &Category) -> Result<()> {
        self.subcategory_id = None;
        self.category_id = category.id.clone();

        sqlx::query!(
            "UPDATE account_transaction SET category_id = ?, subcategory_id = NULL WHERE id = ?;",
            self.category_id,
            self.id
        )
        .execute(&mut *db)
        .await?;

        self.mark_edited(db).await?;

        Ok(())
    }

    /// Sets the transaction's subcategory. This can fail if the subcategory does not match the existing category.
    pub async fn set_subcategory(
        &mut self,
        db: &mut DBImpl,
        subcategory: Option<&Subcategory>,
    ) -> Result<()> {
        if let Some(given_subcategory) = subcategory {
            if given_subcategory.category_id != self.category_id {
                Err(Error::InvalidSubcategory)?;
            }
        }

        self.subcategory_id = subcategory.map(|x| x.id.clone());

        sqlx::query!(
            "UPDATE account_transaction SET subcategory_id = ? WHERE id = ?;",
            self.subcategory_id,
            self.id
        )
        .execute(&mut *db)
        .await?;

        self.mark_edited(db).await?;

        Ok(())
    }

    /// Sets the category and subcategory at the same time. This can fail if the category/subcategory combination is invalid.
    pub async fn set_category_and_subcategory(
        &mut self,
        db: &mut DBImpl,
        category: &Category,
        subcategory: Option<&Subcategory>,
    ) -> Result<()> {
        if let Some(given_subcategory) = subcategory {
            if given_subcategory.category_id != category.id {
                Err(Error::InvalidSubcategory)?;
            }
        }

        self.category_id = category.id.clone();
        self.subcategory_id = subcategory.map(|x| x.id.clone());

        sqlx::query!(
            "UPDATE account_transaction SET category_id = ?, subcategory_id = ? WHERE id = ?;",
            self.category_id,
            self.subcategory_id,
            self.id
        )
        .execute(&mut *db)
        .await?;

        self.mark_edited(db).await?;

        Ok(())
    }

    /// Deletes the account transaction from the database.
    pub async fn delete(self, db: &mut DBImpl) -> Result<()> {
        sqlx::query!("DELETE FROM account_transaction WHERE id = ?;", self.id)
            .execute(&mut *db)
            .await?;

        Ok(())
    }
}

/// Account transaction tests.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AccountType, TestDB};

    #[tokio::test]
    async fn test_account_transaction() {
        // Init
        let mut db = TestDB::new().await.unwrap();

        // Create
        let mut account1 = Account::create(
            &mut db,
            AccountType::RetirementAccount,
            "My Retirement Account",
            "",
        )
        .await
        .unwrap();
        let mut account2 = Account::create(&mut db, AccountType::Property, "My Property", "")
            .await
            .unwrap();
        let institution1 = Institution::create(&mut db, "IHOP", "Internation House of Pancakes")
            .await
            .unwrap();
        let institution2 = Institution::create(&mut db, "Another institution", "")
            .await
            .unwrap();
        let category1 = Category::create(&mut db, "Category #1", "").await.unwrap();
        let category2 = Category::create(&mut db, "Category #2", "").await.unwrap();
        let subcategory1 = Subcategory::create(&mut db, &category1, "Subcategory #1", "")
            .await
            .unwrap();
        let subcategory2 = Subcategory::create(&mut db, &category2, "Subcategory #2", "")
            .await
            .unwrap();
        let mut transaction1 = AccountTransaction::create(
            &mut db,
            &mut account1,
            "Breakfast",
            "Breakfast at IHOP",
            16.75,
            TransactionType::Debit,
            &institution1,
            NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
            &category1,
            Some(&subcategory1),
        )
        .await
        .unwrap();
        let mut transaction2 = AccountTransaction::create(
            &mut db,
            &mut account2,
            "Another transaction",
            "",
            20.00,
            TransactionType::Credit,
            &institution2,
            NaiveDate::from_ymd_opt(2020, 3, 15).unwrap(),
            &category2,
            None,
        )
        .await
        .unwrap();
        assert!(AccountTransaction::create(
            &mut db,
            &mut account1,
            "",
            "",
            0.00,
            TransactionType::Credit,
            &institution1,
            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            &category1,
            Some(&subcategory2)
        )
        .await
        .is_err());
        assert!(AccountTransaction::create(
            &mut db,
            &mut account1,
            "",
            "",
            0.00,
            TransactionType::Debit,
            &institution2,
            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            &category2,
            Some(&subcategory1)
        )
        .await
        .is_err());

        // Get
        let transaction3 = AccountTransaction::get(&mut db, &transaction1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(transaction3, transaction1);
        let transaction4 = AccountTransaction::get(&mut db, &transaction2.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(transaction4, transaction2);
        assert!(AccountTransaction::get(&mut db, "")
            .await
            .unwrap()
            .is_none());

        // List
        let transactions1 = AccountTransaction::list(&mut db).await.unwrap();
        assert_eq!(transactions1.len(), 2);
        let transaction5 = transactions1
            .iter()
            .find(|x| x.id == transaction1.id)
            .unwrap();
        assert_eq!(transaction5, &transaction1);
        let transaction6 = transactions1
            .iter()
            .find(|x| x.id == transaction2.id)
            .unwrap();
        assert_eq!(transaction6, &transaction2);

        // List within account
        let transactions2 = AccountTransaction::list_within(&mut db, &account1)
            .await
            .unwrap();
        assert_eq!(transactions2.len(), 1);
        assert_eq!(transactions2[0], transaction1);
        let transactions3 = AccountTransaction::list_within(&mut db, &account2)
            .await
            .unwrap();
        assert_eq!(transactions3.len(), 1);
        assert_eq!(transactions3[0], transaction2);

        // Get account
        let account3 = transaction1.get_account(&mut db).await.unwrap();
        assert_eq!(account3, account1);
        let account4 = transaction2.get_account(&mut db).await.unwrap();
        assert_eq!(account4, account2);

        // Get transaction type
        let transaction_type1 = transaction1.get_transaction_type();
        assert_eq!(transaction_type1, TransactionType::Debit);
        let transaction_type2 = transaction2.get_transaction_type();
        assert_eq!(transaction_type2, TransactionType::Credit);

        // Get institution
        let institution3 = transaction1.get_institution(&mut db).await.unwrap();
        assert_eq!(institution3, institution1);
        let institution4 = transaction2.get_institution(&mut db).await.unwrap();
        assert_eq!(institution4, institution2);

        // Get date
        let date1 = transaction1.get_date();
        assert_eq!(date1, NaiveDate::from_ymd_opt(2023, 4, 1).unwrap());
        let date2 = transaction2.get_date();
        assert_eq!(date2, NaiveDate::from_ymd_opt(2020, 3, 15).unwrap());

        // Get category
        let category3 = transaction1.get_category(&mut db).await.unwrap();
        assert_eq!(category3, category1);
        let category4 = transaction2.get_category(&mut db).await.unwrap();
        assert_eq!(category4, category2);

        // Get subcategory
        let subcategory3 = transaction1
            .get_subcategory(&mut db)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(subcategory3, subcategory1);
        assert!(transaction2
            .get_subcategory(&mut db)
            .await
            .unwrap()
            .is_none());

        // Mark edited
        assert!(transaction1.edited_at.is_none());
        transaction1.mark_edited(&mut db).await.unwrap();
        assert!(transaction1.edited_at.is_some());
        assert_ne!(transaction1, transaction3);

        // Mark reconciled
        assert!(transaction2.reconciled_at.is_none());
        transaction2.mark_reconciled(&mut db).await.unwrap();
        assert!(transaction2.reconciled_at.is_some());
        assert_ne!(transaction2, transaction4);

        // Set account
        assert_eq!(transaction1.account_id, account1.id);
        transaction1.set_account(&mut db, &account2).await.unwrap();
        assert_eq!(transaction1.account_id, account2.id);
        let transaction7 = AccountTransaction::get(&mut db, &transaction1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(transaction7, transaction1);

        // Set name
        transaction1
            .set_name(&mut db, "New transaction name")
            .await
            .unwrap();
        assert_eq!(&transaction1.name, "New transaction name");
        let transaction8 = AccountTransaction::get(&mut db, &transaction1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(transaction8, transaction1);

        // Set description
        transaction1
            .set_description(&mut db, "New transaction description")
            .await
            .unwrap();
        assert_eq!(
            transaction1.description.as_ref().unwrap().as_str(),
            "New transaction description"
        );
        let transaction9 = AccountTransaction::get(&mut db, &transaction1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(transaction9, transaction1);

        // Set amount
        transaction1.set_amount(&mut db, 0.01).await.unwrap();
        assert_eq!(transaction1.amount, 0.01);
        let transaction10 = AccountTransaction::get(&mut db, &transaction1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(transaction10, transaction1);

        // Set date
        transaction1
            .set_date(&mut db, NaiveDate::from_ymd_opt(2020, 6, 27).unwrap())
            .await
            .unwrap();
        assert_eq!(
            transaction1.transaction_date.date(),
            NaiveDate::from_ymd_opt(2020, 6, 27).unwrap()
        );
        let transaction11 = AccountTransaction::get(&mut db, &transaction1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(transaction11, transaction1);

        // Set category
        transaction1
            .set_category(&mut db, &category2)
            .await
            .unwrap();
        assert_eq!(transaction1.category_id, category2.id);
        assert!(transaction1.subcategory_id.is_none());
        let transaction12 = AccountTransaction::get(&mut db, &transaction1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(transaction12, transaction1);

        // Set subcategory
        transaction1
            .set_subcategory(&mut db, Some(&subcategory2))
            .await
            .unwrap();
        assert_eq!(
            transaction1.subcategory_id.as_ref().unwrap(),
            &subcategory2.id
        );
        let transaction13 = AccountTransaction::get(&mut db, &transaction1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(transaction13, transaction1);
        transaction1.set_subcategory(&mut db, None).await.unwrap();
        assert_eq!(transaction1.subcategory_id, None);
        let transaction14 = AccountTransaction::get(&mut db, &transaction1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(transaction14, transaction1);
        assert!(transaction1
            .set_subcategory(&mut db, Some(&subcategory1))
            .await
            .is_err());

        // Set category and subcategory
        transaction1
            .set_category_and_subcategory(&mut db, &category1, Some(&subcategory1))
            .await
            .unwrap();
        assert_eq!(transaction1.category_id, category1.id);
        assert_eq!(
            transaction1.subcategory_id.as_ref().unwrap(),
            &subcategory1.id
        );
        let transaction15 = AccountTransaction::get(&mut db, &transaction1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(transaction15, transaction1);
        transaction1
            .set_category_and_subcategory(&mut db, &category2, None)
            .await
            .unwrap();
        assert_eq!(transaction1.category_id, category2.id);
        assert_eq!(transaction1.subcategory_id, None);
        let transaction16 = AccountTransaction::get(&mut db, &transaction1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(transaction16, transaction1);
        assert!(transaction1
            .set_category_and_subcategory(&mut db, &category1, Some(&subcategory2))
            .await
            .is_err());

        // Delete
        let transaction_id1 = transaction1.id.clone();
        assert!(AccountTransaction::get(&mut db, &transaction_id1)
            .await
            .unwrap()
            .is_some());
        transaction1.delete(&mut db).await.unwrap();
        assert!(AccountTransaction::get(&mut db, &transaction_id1)
            .await
            .unwrap()
            .is_none());
        let transaction_id2 = transaction2.id.clone();
        assert!(AccountTransaction::get(&mut db, &transaction_id2)
            .await
            .unwrap()
            .is_some());
        transaction2.delete(&mut db).await.unwrap();
        assert!(AccountTransaction::get(&mut db, &transaction_id2)
            .await
            .unwrap()
            .is_none());

        // Clean up
        db.delete().await.unwrap();
    }
}
