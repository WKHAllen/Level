use crate::{AccountTransaction, Tag, DB};
use chrono::NaiveDateTime;

/// A representation of a link between transactions and tags in the database.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountTransactionTag {
    /// The ID of the account transaction.
    pub account_transaction_id: String,
    /// The ID of the tag.
    pub tag_id: String,
    /// When the account transaction tag was created.
    pub created_at: NaiveDateTime,
}

impl AccountTransactionTag {
    /// Create a new account transaction tag.
    pub async fn create(db: &DB, account_transaction: &AccountTransaction, tag: &Tag) -> Self {
        sqlx::query!(
            "INSERT INTO account_transaction_tag (account_transaction_id, tag_id) VALUES (?, ?)",
            account_transaction.id,
            tag.id
        )
        .execute(&**db)
        .await
        .unwrap();

        Self::get(&db, &account_transaction, &tag).await.unwrap()
    }

    /// Gets an account transaction tag from the database.
    pub async fn get(db: &DB, account_transaction: &AccountTransaction, tag: &Tag) -> Option<Self> {
        sqlx::query_as!(Self, "SELECT * FROM account_transaction_tag WHERE account_transaction_id = ? AND tag_id = ?;", account_transaction.id, tag.id)
            .fetch_optional(&**db)
            .await
            .unwrap()
    }

    /// Checks if an account transaction/tag link exists.
    pub async fn exists(db: &DB, account_transaction: &AccountTransaction, tag: &Tag) -> bool {
        Self::get(&db, &account_transaction, &tag).await.is_some()
    }

    /// Lists all account transaction tags in the database.
    pub async fn list(db: &DB) -> Vec<Self> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM account_transaction_tag ORDER BY created_at;"
        )
        .fetch_all(&**db)
        .await
        .unwrap()
    }

    /// Lists account transaction tags corresponding to a given account transaction.
    pub async fn list_by_transaction(
        db: &DB,
        account_transaction: &AccountTransaction,
    ) -> Vec<Self> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM account_transaction_tag WHERE account_transaction_id = ? ORDER BY created_at;",
            account_transaction.id
        )
        .fetch_all(&**db)
        .await
        .unwrap()
    }

    /// Lists account transaction tags corresponding to a given tag.
    pub async fn list_by_tag(db: &DB, tag: &Tag) -> Vec<Self> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM account_transaction_tag WHERE tag_id = ? ORDER BY created_at;",
            tag.id
        )
        .fetch_all(&**db)
        .await
        .unwrap()
    }

    /// Gets the associated account transaction.
    pub async fn get_account_transaction(&self, db: &DB) -> AccountTransaction {
        AccountTransaction::get(&db, &self.account_transaction_id)
            .await
            .unwrap()
    }

    /// Gets the associated tag.
    pub async fn get_tag(&self, db: &DB) -> Tag {
        Tag::get(&db, &self.tag_id).await.unwrap()
    }

    /// Deletes the account transaction tag from the database.
    pub async fn delete(self, db: &DB) {
        sqlx::query!(
            "DELETE FROM account_transaction_tag WHERE account_transaction_id = ? AND tag_id = ?;",
            self.account_transaction_id,
            self.tag_id
        )
        .execute(&**db)
        .await
        .unwrap();
    }
}

/// Account transaction tag tests.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Account, AccountType, Category, TestDB};
    use chrono::NaiveDate;

    #[tokio::test]
    async fn test_account_transaction_tag() {
        // Init
        let db = TestDB::new().await.unwrap();

        // Create
        let mut account = Account::create(&db, AccountType::BankAccount, "My account", "").await;
        let category = Category::create(&db, "My category", "").await;
        let transaction1 = AccountTransaction::create(
            &db,
            &mut account,
            "Transaction 1",
            "",
            0.01,
            NaiveDate::from_ymd_opt(2023, 04, 01).unwrap(),
            &category,
            None,
        )
        .await
        .unwrap();
        let transaction2 = AccountTransaction::create(
            &db,
            &mut account,
            "Transaction 2",
            "",
            0.99,
            NaiveDate::from_ymd_opt(2023, 03, 31).unwrap(),
            &category,
            None,
        )
        .await
        .unwrap();
        let tag1 = Tag::create(&db, "Tag 1", "").await;
        let tag2 = Tag::create(&db, "Tag 2", "").await;
        let transaction_tag1 = AccountTransactionTag::create(&db, &transaction1, &tag1).await;
        let transaction_tag2 = AccountTransactionTag::create(&db, &transaction2, &tag2).await;

        // Get
        let transaction_tag3 = AccountTransactionTag::get(&db, &transaction1, &tag1)
            .await
            .unwrap();
        assert_eq!(transaction_tag3, transaction_tag1);
        let transaction_tag4 = AccountTransactionTag::get(&db, &transaction2, &tag2)
            .await
            .unwrap();
        assert_eq!(transaction_tag4, transaction_tag2);

        // Exists
        assert!(AccountTransactionTag::exists(&db, &transaction1, &tag1).await);
        assert!(AccountTransactionTag::exists(&db, &transaction2, &tag2).await);
        assert!(!AccountTransactionTag::exists(&db, &transaction1, &tag2).await);
        assert!(!AccountTransactionTag::exists(&db, &transaction2, &tag1).await);

        // List
        let transaction_tags1 = AccountTransactionTag::list(&db).await;
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
        let transaction_tags2 =
            AccountTransactionTag::list_by_transaction(&db, &transaction1).await;
        assert_eq!(transaction_tags2.len(), 1);
        assert_eq!(transaction_tags2[0], transaction_tag1);
        let transaction_tags3 =
            AccountTransactionTag::list_by_transaction(&db, &transaction2).await;
        assert_eq!(transaction_tags3.len(), 1);
        assert_eq!(transaction_tags3[0], transaction_tag2);

        // List by tag
        let transaction_tags4 = AccountTransactionTag::list_by_tag(&db, &tag1).await;
        assert_eq!(transaction_tags4.len(), 1);
        assert_eq!(transaction_tags4[0], transaction_tag1);
        let transaction_tags5 = AccountTransactionTag::list_by_tag(&db, &tag2).await;
        assert_eq!(transaction_tags5.len(), 1);
        assert_eq!(transaction_tags5[0], transaction_tag2);

        // Get account transaction
        let transaction3 = transaction_tag1.get_account_transaction(&db).await;
        assert_eq!(transaction3, transaction1);
        let transaction4 = transaction_tag2.get_account_transaction(&db).await;
        assert_eq!(transaction4, transaction2);

        // Get tag
        let tag3 = transaction_tag1.get_tag(&db).await;
        assert_eq!(tag3, tag1);
        let tag4 = transaction_tag2.get_tag(&db).await;
        assert_eq!(tag4, tag2);

        // Delete
        assert!(AccountTransactionTag::exists(&db, &transaction1, &tag1).await);
        transaction_tag1.delete(&db).await;
        assert!(!AccountTransactionTag::exists(&db, &transaction1, &tag1).await);
        assert!(AccountTransactionTag::exists(&db, &transaction2, &tag2).await);
        transaction_tag2.delete(&db).await;
        assert!(!AccountTransactionTag::exists(&db, &transaction2, &tag2).await);

        // Clean up
        db.delete().await.unwrap();
    }
}
