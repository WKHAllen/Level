use crate::{new_id, DB};

/// A representation of a quote in the database.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Quote {
    /// The quote's identifier.
    pub id: String,
    /// The quote itself.
    pub quote: String,
}

impl Quote {
    /// Create a new quote in the database.
    pub async fn create(db: &DB, quote: &str) -> Self {
        let id = new_id();

        sqlx::query!("INSERT INTO quotes VALUES (?, ?);", id, quote)
            .execute(&**db)
            .await
            .unwrap();

        Self::get(&db, &id).await.unwrap()
    }

    /// Retrieve a quote from the database.
    pub async fn get(db: &DB, id: &str) -> Option<Self> {
        sqlx::query_as!(Self, "SELECT * FROM quotes WHERE id = ?;", id)
            .fetch_optional(&**db)
            .await
            .unwrap()
    }

    /// List all quotes in the database.
    pub async fn list(db: &DB) -> Vec<Quote> {
        sqlx::query_as!(Self, "SELECT * FROM quotes;")
            .fetch_all(&**db)
            .await
            .unwrap()
    }

    /// Set the quote's value, both in memory and in the database.
    pub async fn set(&mut self, db: &DB, new_quote: &str) {
        self.quote = new_quote.to_owned();

        sqlx::query!(
            "UPDATE quotes SET quote = ? WHERE id = ?;",
            self.quote,
            self.id
        )
        .execute(&**db)
        .await
        .unwrap();
    }

    /// Delete the quote from the database, consuming it in memory.
    pub async fn delete(self, db: &DB) {
        sqlx::query!("DELETE FROM quotes WHERE id = ?;", self.id)
            .execute(&**db)
            .await
            .unwrap();
    }
}
