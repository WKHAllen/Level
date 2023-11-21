use crate::{new_id, DBImpl};
use async_trait::async_trait;
use backend_common::Result;
use common::*;

/// The database implementation of the category model.
#[async_trait]
pub trait DBCategory: Sized {
    /// Creates a new category.
    async fn create(db: &mut DBImpl, name: &str, description: &str) -> Result<Self>;

    /// Gets a category from the database.
    async fn get(db: &mut DBImpl, id: &str) -> Result<Option<Self>>;

    /// Gets a category from the database by name.
    async fn get_by_name(db: &mut DBImpl, name: &str) -> Result<Option<Self>>;

    /// Lists all categories in the database.
    async fn list(db: &mut DBImpl) -> Result<Vec<Self>>;

    /// Sets the category name.
    async fn set_name(&mut self, db: &mut DBImpl, name: &str) -> Result<()>;

    /// Sets the category description.
    async fn set_description(&mut self, db: &mut DBImpl, description: &str) -> Result<()>;

    /// Deletes the category from the database.
    async fn delete(self, db: &mut DBImpl) -> Result<()>;
}

#[async_trait]
impl DBCategory for Category {
    async fn create(db: &mut DBImpl, name: &str, description: &str) -> Result<Self> {
        let id = new_id();

        sqlx::query!(
            "INSERT INTO category (id, name, description) VALUES (?, ?, ?);",
            id,
            name,
            description
        )
        .execute(&mut *db)
        .await?;

        Self::get(db, &id).await.map(|x| x.unwrap())
    }

    async fn get(db: &mut DBImpl, id: &str) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM category WHERE id = ?;", id)
                .fetch_optional(&mut *db)
                .await?,
        )
    }

    async fn get_by_name(db: &mut DBImpl, name: &str) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM category WHERE name = ?;", name)
                .fetch_optional(&mut *db)
                .await?,
        )
    }

    async fn list(db: &mut DBImpl) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM category ORDER BY name;")
                .fetch_all(&mut *db)
                .await?,
        )
    }

    async fn set_name(&mut self, db: &mut DBImpl, name: &str) -> Result<()> {
        self.name = name.to_owned();

        sqlx::query!(
            "UPDATE category SET name = ? WHERE id = ?;",
            self.name,
            self.id
        )
        .execute(&mut *db)
        .await?;

        Ok(())
    }

    async fn set_description(&mut self, db: &mut DBImpl, description: &str) -> Result<()> {
        self.description = Some(description.to_owned());

        sqlx::query!(
            "UPDATE category SET description = ? WHERE id = ?;",
            self.description,
            self.id
        )
        .execute(&mut *db)
        .await?;

        Ok(())
    }

    async fn delete(self, db: &mut DBImpl) -> Result<()> {
        sqlx::query!("DELETE FROM category WHERE id = ?;", self.id)
            .execute(&mut *db)
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
