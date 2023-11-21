use crate::{DBAccountTransaction, DBImpl, DBTag};
use async_trait::async_trait;
use backend_common::Result;
use common::*;

/// The database implementation of the account transaction tag model.
#[async_trait]
pub trait DBAccountTransactionTag: Sized {
    /// Create a new account transaction tag.
    async fn create(
        db: &mut DBImpl,
        account_transaction: &AccountTransaction,
        tag: &Tag,
    ) -> Result<Self>;

    /// Gets an account transaction tag from the database.
    async fn get(
        db: &mut DBImpl,
        account_transaction: &AccountTransaction,
        tag: &Tag,
    ) -> Result<Option<Self>>;

    /// Checks if an account transaction/tag link exists.
    async fn exists(
        db: &mut DBImpl,
        account_transaction: &AccountTransaction,
        tag: &Tag,
    ) -> Result<bool>;

    /// Lists all account transaction tags in the database.
    async fn list(db: &mut DBImpl) -> Result<Vec<Self>>;

    /// Lists account transaction tags corresponding to a given account transaction.
    async fn list_by_transaction(
        db: &mut DBImpl,
        account_transaction: &AccountTransaction,
    ) -> Result<Vec<Self>>;

    /// Lists account transaction tags corresponding to a given tag.
    async fn list_by_tag(db: &mut DBImpl, tag: &Tag) -> Result<Vec<Self>>;

    /// Gets the associated account transaction.
    async fn get_account_transaction(&self, db: &mut DBImpl) -> Result<AccountTransaction>;

    /// Gets the associated tag.
    async fn get_tag(&self, db: &mut DBImpl) -> Result<Tag>;

    /// Deletes the account transaction tag from the database.
    async fn delete(self, db: &mut DBImpl) -> Result<()>;
}

#[async_trait]
impl DBAccountTransactionTag for AccountTransactionTag {
    async fn create(
        db: &mut DBImpl,
        account_transaction: &AccountTransaction,
        tag: &Tag,
    ) -> Result<Self> {
        sqlx::query!(
            "INSERT INTO account_transaction_tag (account_transaction_id, tag_id) VALUES (?, ?)",
            account_transaction.id,
            tag.id
        )
        .execute(&mut *db)
        .await?;

        Ok(Self::get(db, account_transaction, tag).await?.unwrap())
    }

    async fn get(
        db: &mut DBImpl,
        account_transaction: &AccountTransaction,
        tag: &Tag,
    ) -> Result<Option<Self>> {
        Ok(sqlx::query_as!(Self, "SELECT * FROM account_transaction_tag WHERE account_transaction_id = ? AND tag_id = ?;", account_transaction.id, tag.id)
            .fetch_optional(&mut *db)
            .await?)
    }

    async fn exists(
        db: &mut DBImpl,
        account_transaction: &AccountTransaction,
        tag: &Tag,
    ) -> Result<bool> {
        Self::get(db, account_transaction, tag)
            .await
            .map(|x| x.is_some())
    }

    async fn list(db: &mut DBImpl) -> Result<Vec<Self>> {
        Ok(sqlx::query_as!(
            Self,
            "SELECT * FROM account_transaction_tag ORDER BY created_at;"
        )
        .fetch_all(&mut *db)
        .await?)
    }

    async fn list_by_transaction(
        db: &mut DBImpl,
        account_transaction: &AccountTransaction,
    ) -> Result<Vec<Self>> {
        Ok(sqlx::query_as!(
            Self,
            "SELECT * FROM account_transaction_tag WHERE account_transaction_id = ? ORDER BY created_at;",
            account_transaction.id
        )
        .fetch_all(&mut *db)
        .await?)
    }

    async fn list_by_tag(db: &mut DBImpl, tag: &Tag) -> Result<Vec<Self>> {
        Ok(sqlx::query_as!(
            Self,
            "SELECT * FROM account_transaction_tag WHERE tag_id = ? ORDER BY created_at;",
            tag.id
        )
        .fetch_all(&mut *db)
        .await?)
    }

    async fn get_account_transaction(&self, db: &mut DBImpl) -> Result<AccountTransaction> {
        AccountTransaction::get(db, &self.account_transaction_id)
            .await
            .map(|x| x.unwrap())
    }

    async fn get_tag(&self, db: &mut DBImpl) -> Result<Tag> {
        Tag::get(db, &self.tag_id).await.map(|x| x.unwrap())
    }

    async fn delete(self, db: &mut DBImpl) -> Result<()> {
        sqlx::query!(
            "DELETE FROM account_transaction_tag WHERE account_transaction_id = ? AND tag_id = ?;",
            self.account_transaction_id,
            self.tag_id
        )
        .execute(&mut *db)
        .await?;

        Ok(())
    }
}

/// Account transaction tag tests.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DBAccount, DBCategory, DBInstitution, TestDB};
    use chrono::NaiveDate;

    #[tokio::test]
    async fn test_account_transaction_tag() {
        // Init
        let mut db = TestDB::new().await.unwrap();

        // Create
        let mut account = Account::create(&mut db, AccountType::BankAccount, "My account", "")
            .await
            .unwrap();
        let institution = Institution::create(&mut db, "My institution", "")
            .await
            .unwrap();
        let category = Category::create(&mut db, "My category", "").await.unwrap();
        let transaction1 = AccountTransaction::create(
            &mut db,
            &mut account,
            "Transaction 1",
            "",
            0.01,
            TransactionType::Credit,
            &institution,
            NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
            &category,
            None,
        )
        .await
        .unwrap();
        let transaction2 = AccountTransaction::create(
            &mut db,
            &mut account,
            "Transaction 2",
            "",
            0.99,
            TransactionType::Debit,
            &institution,
            NaiveDate::from_ymd_opt(2023, 3, 31).unwrap(),
            &category,
            None,
        )
        .await
        .unwrap();
        let tag1 = Tag::create(&mut db, "Tag 1", "").await.unwrap();
        let tag2 = Tag::create(&mut db, "Tag 2", "").await.unwrap();
        let transaction_tag1 = AccountTransactionTag::create(&mut db, &transaction1, &tag1)
            .await
            .unwrap();
        let transaction_tag2 = AccountTransactionTag::create(&mut db, &transaction2, &tag2)
            .await
            .unwrap();

        // Get
        let transaction_tag3 = AccountTransactionTag::get(&mut db, &transaction1, &tag1)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(transaction_tag3, transaction_tag1);
        let transaction_tag4 = AccountTransactionTag::get(&mut db, &transaction2, &tag2)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(transaction_tag4, transaction_tag2);

        // Exists
        assert!(AccountTransactionTag::exists(&mut db, &transaction1, &tag1)
            .await
            .unwrap());
        assert!(AccountTransactionTag::exists(&mut db, &transaction2, &tag2)
            .await
            .unwrap());
        assert!(
            !AccountTransactionTag::exists(&mut db, &transaction1, &tag2)
                .await
                .unwrap()
        );
        assert!(
            !AccountTransactionTag::exists(&mut db, &transaction2, &tag1)
                .await
                .unwrap()
        );

        // List
        let transaction_tags1 = AccountTransactionTag::list(&mut db).await.unwrap();
        assert_eq!(transaction_tags1.len(), 2);
        let transaction_tag5 = transaction_tags1
            .iter()
            .find(|x| x.account_transaction_id == transaction1.id && x.tag_id == tag1.id)
            .unwrap();
        assert_eq!(transaction_tag5, &transaction_tag1);
        let transaction_tag6 = transaction_tags1
            .iter()
            .find(|x| x.account_transaction_id == transaction2.id && x.tag_id == tag2.id)
            .unwrap();
        assert_eq!(transaction_tag6, &transaction_tag2);

        // List by transaction
        let transaction_tags2 = AccountTransactionTag::list_by_transaction(&mut db, &transaction1)
            .await
            .unwrap();
        assert_eq!(transaction_tags2.len(), 1);
        assert_eq!(transaction_tags2[0], transaction_tag1);
        let transaction_tags3 = AccountTransactionTag::list_by_transaction(&mut db, &transaction2)
            .await
            .unwrap();
        assert_eq!(transaction_tags3.len(), 1);
        assert_eq!(transaction_tags3[0], transaction_tag2);

        // List by tag
        let transaction_tags4 = AccountTransactionTag::list_by_tag(&mut db, &tag1)
            .await
            .unwrap();
        assert_eq!(transaction_tags4.len(), 1);
        assert_eq!(transaction_tags4[0], transaction_tag1);
        let transaction_tags5 = AccountTransactionTag::list_by_tag(&mut db, &tag2)
            .await
            .unwrap();
        assert_eq!(transaction_tags5.len(), 1);
        assert_eq!(transaction_tags5[0], transaction_tag2);

        // Get account transaction
        let transaction3 = transaction_tag1
            .get_account_transaction(&mut db)
            .await
            .unwrap();
        assert_eq!(transaction3, transaction1);
        let transaction4 = transaction_tag2
            .get_account_transaction(&mut db)
            .await
            .unwrap();
        assert_eq!(transaction4, transaction2);

        // Get tag
        let tag3 = transaction_tag1.get_tag(&mut db).await.unwrap();
        assert_eq!(tag3, tag1);
        let tag4 = transaction_tag2.get_tag(&mut db).await.unwrap();
        assert_eq!(tag4, tag2);

        // Delete
        assert!(AccountTransactionTag::exists(&mut db, &transaction1, &tag1)
            .await
            .unwrap());
        transaction_tag1.delete(&mut db).await.unwrap();
        assert!(
            !AccountTransactionTag::exists(&mut db, &transaction1, &tag1)
                .await
                .unwrap()
        );
        assert!(AccountTransactionTag::exists(&mut db, &transaction2, &tag2)
            .await
            .unwrap());
        transaction_tag2.delete(&mut db).await.unwrap();
        assert!(
            !AccountTransactionTag::exists(&mut db, &transaction2, &tag2)
                .await
                .unwrap()
        );

        // Clean up
        db.delete().await.unwrap();
    }
}
