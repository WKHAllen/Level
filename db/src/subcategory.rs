use crate::{new_id, Category, DB};
use backend_common::Result;
use chrono::NaiveDateTime;

/// A representation of a subcategory in the database.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Subcategory {
    /// The subcategory's identifier.
    pub id: String,
    /// The ID of the category in which the subcategory exists.
    pub category_id: String,
    /// The name of the category.
    pub name: String,
    /// A description of the category.
    pub description: Option<String>,
    /// When the category was created.
    pub created_at: NaiveDateTime,
}

impl Subcategory {
    /// Create a new subcategory.
    pub async fn create(
        db: &mut DB,
        category: &Category,
        name: &str,
        description: &str,
    ) -> Result<Self> {
        let id = new_id();

        sqlx::query!(
            "INSERT INTO subcategory (id, category_id, name, description) VALUES (?, ?, ?, ?);",
            id,
            category.id,
            name,
            description
        )
        .execute(&mut **db)
        .await?;

        Self::get(db, &id).await.map(|x| x.unwrap())
    }

    /// Gets a subcategory from the database.
    pub async fn get(db: &mut DB, id: &str) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM subcategory WHERE id = ?;", id)
                .fetch_optional(&mut **db)
                .await?,
        )
    }

    /// Gets a subcategory from the database by name.
    pub async fn get_by_name(db: &mut DB, name: &str) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM subcategory WHERE name = ?;", name)
                .fetch_optional(&mut **db)
                .await?,
        )
    }

    /// Lists all subcategories in the database.
    pub async fn list(db: &mut DB) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM subcategory ORDER BY name;")
                .fetch_all(&mut **db)
                .await?,
        )
    }

    /// Lists all subcategories within a given category.
    pub async fn list_within(db: &mut DB, category: &Category) -> Result<Vec<Self>> {
        Ok(sqlx::query_as!(
            Self,
            "SELECT * FROM subcategory WHERE category_id = ? ORDER BY name;",
            category.id
        )
        .fetch_all(&mut **db)
        .await?)
    }

    /// Gets the category in which the subcategory exists.
    pub async fn get_category(&self, db: &mut DB) -> Result<Category> {
        Category::get(db, &self.category_id)
            .await
            .map(|x| x.unwrap())
    }

    /// Sets the subcategory name.
    pub async fn set_name(&mut self, db: &mut DB, name: &str) -> Result<()> {
        self.name = name.to_owned();

        sqlx::query!(
            "UPDATE subcategory SET name = ? WHERE id = ?;",
            self.name,
            self.id
        )
        .execute(&mut **db)
        .await?;

        Ok(())
    }

    /// Sets the subcategory description.
    pub async fn set_description(&mut self, db: &mut DB, description: &str) -> Result<()> {
        self.description = Some(description.to_owned());

        sqlx::query!(
            "UPDATE subcategory SET description = ? WHERE id = ?;",
            self.description,
            self.id
        )
        .execute(&mut **db)
        .await?;

        Ok(())
    }

    /// Deletes the subcategory from the database.
    pub async fn delete(self, db: &mut DB) -> Result<()> {
        sqlx::query!("DELETE FROM subcategory WHERE id = ?;", self.id)
            .execute(&mut **db)
            .await?;

        Ok(())
    }
}

/// Subcategory tests.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::TestDB;

    #[tokio::test]
    async fn test_subcategory() {
        // Init
        let mut db = TestDB::new().await.unwrap();

        // Create
        let category1 = Category::create(&mut db, "Category #1", "").await.unwrap();
        let category2 = Category::create(&mut db, "Category #2", "").await.unwrap();
        let mut subcategory1 = Subcategory::create(
            &mut db,
            &category1,
            "Subcategory #1",
            "The first subcategory",
        )
        .await
        .unwrap();
        let subcategory2 = Subcategory::create(
            &mut db,
            &category1,
            "Subcategory #2",
            "The second subcategory",
        )
        .await
        .unwrap();
        let subcategory3 = Subcategory::create(
            &mut db,
            &category2,
            "Subcategory #3",
            "The third subcategory",
        )
        .await
        .unwrap();

        // Get
        let subcategory4 = Subcategory::get(&mut db, &subcategory1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(subcategory4, subcategory1);
        let subcategory5 = Subcategory::get(&mut db, &subcategory2.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(subcategory5, subcategory2);
        let subcategory6 = Subcategory::get(&mut db, &subcategory3.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(subcategory6, subcategory3);
        assert!(Subcategory::get(&mut db, "").await.unwrap().is_none());

        // Get by name
        let subcategory7 = Subcategory::get_by_name(&mut db, &subcategory1.name)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(subcategory7, subcategory1);
        let subcategory8 = Subcategory::get_by_name(&mut db, &subcategory2.name)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(subcategory8, subcategory2);
        let subcategory9 = Subcategory::get_by_name(&mut db, &subcategory3.name)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(subcategory9, subcategory3);
        assert!(Subcategory::get_by_name(&mut db, "Invalid subcategory")
            .await
            .unwrap()
            .is_none());

        // List
        let subcategories1 = Subcategory::list(&mut db).await.unwrap();
        assert_eq!(subcategories1.len(), 3);
        let subcategory10 = subcategories1
            .iter()
            .find(|x| x.id == subcategory1.id)
            .unwrap();
        assert_eq!(subcategory10, &subcategory1);
        let subcategory11 = subcategories1
            .iter()
            .find(|x| x.id == subcategory2.id)
            .unwrap();
        assert_eq!(subcategory11, &subcategory2);
        let subcategory12 = subcategories1
            .iter()
            .find(|x| x.id == subcategory3.id)
            .unwrap();
        assert_eq!(subcategory12, &subcategory3);

        // List within category
        let subcategories2 = Subcategory::list_within(&mut db, &category1).await.unwrap();
        assert_eq!(subcategories2.len(), 2);
        let subcategory13 = subcategories2
            .iter()
            .find(|x| x.id == subcategory1.id)
            .unwrap();
        assert_eq!(subcategory13, &subcategory1);
        let subcategory14 = subcategories2
            .iter()
            .find(|x| x.id == subcategory2.id)
            .unwrap();
        assert_eq!(subcategory14, &subcategory2);
        let subcategories3 = Subcategory::list_within(&mut db, &category2).await.unwrap();
        assert_eq!(subcategories3.len(), 1);
        assert_eq!(subcategories3[0], subcategory3);

        // Get category
        assert_eq!(subcategory1.get_category(&mut db).await.unwrap(), category1);
        assert_eq!(subcategory2.get_category(&mut db).await.unwrap(), category1);
        assert_eq!(subcategory3.get_category(&mut db).await.unwrap(), category2);

        // // Set category
        // assert_eq!(subcategory1.category_id, category1.id);
        // subcategory1.set_category(&mut db, &category2).await;
        // assert_eq!(subcategory1.category_id, category2.id);
        // assert_eq!(subcategory1.get_category(&mut db).await, category2);

        // Set name
        subcategory1.set_name(&mut db, "name 1").await.unwrap();
        assert_eq!(&subcategory1.name, "name 1");
        let subcategory15 = Subcategory::get(&mut db, &subcategory1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(subcategory15, subcategory1);

        // Set description
        subcategory1
            .set_description(&mut db, "description 1")
            .await
            .unwrap();
        assert_eq!(subcategory1.description.as_ref().unwrap(), "description 1");
        let subcategory16 = Subcategory::get(&mut db, &subcategory1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(subcategory16, subcategory1);

        // Delete
        let subcategory_id1 = subcategory1.id.clone();
        assert!(Subcategory::get(&mut db, &subcategory_id1)
            .await
            .unwrap()
            .is_some());
        subcategory1.delete(&mut db).await.unwrap();
        assert!(Subcategory::get(&mut db, &subcategory_id1)
            .await
            .unwrap()
            .is_none());
        let subcategory_id2 = subcategory2.id.clone();
        assert!(Subcategory::get(&mut db, &subcategory_id2)
            .await
            .unwrap()
            .is_some());
        subcategory2.delete(&mut db).await.unwrap();
        assert!(Subcategory::get(&mut db, &subcategory_id2)
            .await
            .unwrap()
            .is_none());
        let subcategory_id3 = subcategory3.id.clone();
        assert!(Subcategory::get(&mut db, &subcategory_id3)
            .await
            .unwrap()
            .is_some());
        subcategory3.delete(&mut db).await.unwrap();
        assert!(Subcategory::get(&mut db, &subcategory_id3)
            .await
            .unwrap()
            .is_none());

        // Clean up
        db.delete().await.unwrap();
    }
}
