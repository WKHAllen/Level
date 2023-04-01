use crate::{new_id, DB};
use chrono::NaiveDateTime;

/// A representation of a category in the database.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Category {
    /// The category's identifier.
    pub id: String,
    /// The name of the category.
    pub name: String,
    /// A description of the category.
    pub description: Option<String>,
    /// When the category was created.
    pub created_at: NaiveDateTime,
}

impl Category {
    /// Creates a new category.
    pub async fn create(db: &DB, name: &str, description: &str) -> Self {
        let id = new_id();

        sqlx::query!(
            "INSERT INTO category (id, name, description) VALUES (?, ?, ?);",
            id,
            name,
            description
        )
        .execute(&**db)
        .await
        .unwrap();

        Self::get(&db, &id).await.unwrap()
    }

    /// Gets a category from the database.
    pub async fn get(db: &DB, id: &str) -> Option<Self> {
        sqlx::query_as!(Self, "SELECT * FROM category WHERE id = ?;", id)
            .fetch_optional(&**db)
            .await
            .unwrap()
    }

    /// Gets a category from the database by name.
    pub async fn get_by_name(db: &DB, name: &str) -> Option<Self> {
        sqlx::query_as!(Self, "SELECT * FROM category WHERE name = ?;", name)
            .fetch_optional(&**db)
            .await
            .unwrap()
    }

    /// Lists all categories in the database.
    pub async fn list(db: &DB) -> Vec<Self> {
        sqlx::query_as!(Self, "SELECT * FROM category ORDER BY name;")
            .fetch_all(&**db)
            .await
            .unwrap()
    }

    /// Sets the category name.
    pub async fn set_name(&mut self, db: &DB, name: &str) {
        self.name = name.to_owned();

        sqlx::query!(
            "UPDATE category SET name = ? WHERE id = ?;",
            self.name,
            self.id
        )
        .execute(&**db)
        .await
        .unwrap();
    }

    /// Sets the category description.
    pub async fn set_description(&mut self, db: &DB, description: &str) {
        self.description = Some(description.to_owned());

        sqlx::query!(
            "UPDATE category SET description = ? WHERE id = ?;",
            self.description,
            self.id
        )
        .execute(&**db)
        .await
        .unwrap();
    }

    /// Deletes the category from the database.
    pub async fn delete(self, db: &DB) {
        sqlx::query!("DELETE FROM category WHERE id = ?;", self.id)
            .execute(&**db)
            .await
            .unwrap();
    }
}

/// Category tests.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::TestDB;

    #[tokio::test]
    async fn test_category() {
        // Init
        let db = TestDB::new().await.unwrap();

        // Create
        let mut category1 = Category::create(&db, "Category #1", "The first category").await;
        let category2 = Category::create(&db, "Category #2", "The second category").await;
        let category3 = Category::create(&db, "Category #3", "The third category").await;

        // Get
        let category4 = Category::get(&db, &category1.id).await.unwrap();
        assert_eq!(category4, category1);
        let category5 = Category::get(&db, &category2.id).await.unwrap();
        assert_eq!(category5, category2);
        let category6 = Category::get(&db, &category3.id).await.unwrap();
        assert_eq!(category6, category3);
        assert!(Category::get(&db, "").await.is_none());

        // Get by name
        let category7 = Category::get_by_name(&db, &category1.name).await.unwrap();
        assert_eq!(category7, category1);
        let category8 = Category::get_by_name(&db, &category2.name).await.unwrap();
        assert_eq!(category8, category2);
        let category9 = Category::get_by_name(&db, &category3.name).await.unwrap();
        assert_eq!(category9, category3);
        assert!(Category::get_by_name(&db, "Invalid category")
            .await
            .is_none());

        // List
        let categories = Category::list(&db).await;
        assert_eq!(categories.len(), 3);
        let category10 = categories.iter().find(|x| x.id == category1.id).unwrap();
        assert_eq!(category10, &category1);
        let category11 = categories.iter().find(|x| x.id == category2.id).unwrap();
        assert_eq!(category11, &category2);
        let category12 = categories.iter().find(|x| x.id == category3.id).unwrap();
        assert_eq!(category12, &category3);

        // Set name
        category1.set_name(&db, "name 1").await;
        assert_eq!(&category1.name, "name 1");
        let category13 = Category::get(&db, &category1.id).await.unwrap();
        assert_eq!(category13, category1);

        // Set description
        category1.set_description(&db, "description 1").await;
        assert_eq!(category1.description.as_ref().unwrap(), "description 1");
        let category14 = Category::get(&db, &category1.id).await.unwrap();
        assert_eq!(category14, category1);

        // Delete
        let category_id1 = category1.id.clone();
        assert!(Category::get(&db, &category_id1).await.is_some());
        category1.delete(&db).await;
        assert!(Category::get(&db, &category_id1).await.is_none());
        let category_id2 = category2.id.clone();
        assert!(Category::get(&db, &category_id2).await.is_some());
        category2.delete(&db).await;
        assert!(Category::get(&db, &category_id2).await.is_none());
        let category_id3 = category3.id.clone();
        assert!(Category::get(&db, &category_id3).await.is_some());
        category3.delete(&db).await;
        assert!(Category::get(&db, &category_id3).await.is_none());

        // Clean up
        db.delete().await.unwrap();
    }
}
