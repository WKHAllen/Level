use crate::{new_id, DB};
use backend_common::Result;
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
    pub async fn create(db: &mut DB, name: &str, description: &str) -> Result<Self> {
        let id = new_id();

        sqlx::query!(
            "INSERT INTO category (id, name, description) VALUES (?, ?, ?);",
            id,
            name,
            description
        )
        .execute(&mut **db)
        .await?;

        Self::get(db, &id).await.map(|x| x.unwrap())
    }

    /// Gets a category from the database.
    pub async fn get(db: &mut DB, id: &str) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM category WHERE id = ?;", id)
                .fetch_optional(&mut **db)
                .await?,
        )
    }

    /// Gets a category from the database by name.
    pub async fn get_by_name(db: &mut DB, name: &str) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM category WHERE name = ?;", name)
                .fetch_optional(&mut **db)
                .await?,
        )
    }

    /// Lists all categories in the database.
    pub async fn list(db: &mut DB) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM category ORDER BY name;")
                .fetch_all(&mut **db)
                .await?,
        )
    }

    /// Sets the category name.
    pub async fn set_name(&mut self, db: &mut DB, name: &str) -> Result<()> {
        self.name = name.to_owned();

        sqlx::query!(
            "UPDATE category SET name = ? WHERE id = ?;",
            self.name,
            self.id
        )
        .execute(&mut **db)
        .await?;

        Ok(())
    }

    /// Sets the category description.
    pub async fn set_description(&mut self, db: &mut DB, description: &str) -> Result<()> {
        self.description = Some(description.to_owned());

        sqlx::query!(
            "UPDATE category SET description = ? WHERE id = ?;",
            self.description,
            self.id
        )
        .execute(&mut **db)
        .await?;

        Ok(())
    }

    /// Deletes the category from the database.
    pub async fn delete(self, db: &mut DB) -> Result<()> {
        sqlx::query!("DELETE FROM category WHERE id = ?;", self.id)
            .execute(&mut **db)
            .await?;

        Ok(())
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
        let mut db = TestDB::new().await.unwrap();

        // Create
        let mut category1 = Category::create(&mut db, "Category #1", "The first category")
            .await
            .unwrap();
        let category2 = Category::create(&mut db, "Category #2", "The second category")
            .await
            .unwrap();
        let category3 = Category::create(&mut db, "Category #3", "The third category")
            .await
            .unwrap();

        // Get
        let category4 = Category::get(&mut db, &category1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(category4, category1);
        let category5 = Category::get(&mut db, &category2.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(category5, category2);
        let category6 = Category::get(&mut db, &category3.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(category6, category3);
        assert!(Category::get(&mut db, "").await.unwrap().is_none());

        // Get by name
        let category7 = Category::get_by_name(&mut db, &category1.name)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(category7, category1);
        let category8 = Category::get_by_name(&mut db, &category2.name)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(category8, category2);
        let category9 = Category::get_by_name(&mut db, &category3.name)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(category9, category3);
        assert!(Category::get_by_name(&mut db, "Invalid category")
            .await
            .unwrap()
            .is_none());

        // List
        let categories = Category::list(&mut db).await.unwrap();
        assert_eq!(categories.len(), 3);
        let category10 = categories.iter().find(|x| x.id == category1.id).unwrap();
        assert_eq!(category10, &category1);
        let category11 = categories.iter().find(|x| x.id == category2.id).unwrap();
        assert_eq!(category11, &category2);
        let category12 = categories.iter().find(|x| x.id == category3.id).unwrap();
        assert_eq!(category12, &category3);

        // Set name
        category1.set_name(&mut db, "name 1").await.unwrap();
        assert_eq!(&category1.name, "name 1");
        let category13 = Category::get(&mut db, &category1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(category13, category1);

        // Set description
        category1
            .set_description(&mut db, "description 1")
            .await
            .unwrap();
        assert_eq!(category1.description.as_ref().unwrap(), "description 1");
        let category14 = Category::get(&mut db, &category1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(category14, category1);

        // Delete
        let category_id1 = category1.id.clone();
        assert!(Category::get(&mut db, &category_id1)
            .await
            .unwrap()
            .is_some());
        category1.delete(&mut db).await.unwrap();
        assert!(Category::get(&mut db, &category_id1)
            .await
            .unwrap()
            .is_none());
        let category_id2 = category2.id.clone();
        assert!(Category::get(&mut db, &category_id2)
            .await
            .unwrap()
            .is_some());
        category2.delete(&mut db).await.unwrap();
        assert!(Category::get(&mut db, &category_id2)
            .await
            .unwrap()
            .is_none());
        let category_id3 = category3.id.clone();
        assert!(Category::get(&mut db, &category_id3)
            .await
            .unwrap()
            .is_some());
        category3.delete(&mut db).await.unwrap();
        assert!(Category::get(&mut db, &category_id3)
            .await
            .unwrap()
            .is_none());

        // Clean up
        db.delete().await.unwrap();
    }
}
