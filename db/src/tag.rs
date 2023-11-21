use crate::{new_id, DBImpl};
use async_trait::async_trait;
use backend_common::Result;
use common::*;

/// The database implementation of the tag model.
#[async_trait]
pub trait DBTag: Sized {
    /// Creates a new tag.
    async fn create(db: &mut DBImpl, name: &str, description: &str) -> Result<Self>;

    /// Gets a tag from the database.
    async fn get(db: &mut DBImpl, id: &str) -> Result<Option<Self>>;

    /// Lists all tags in the database.
    async fn list(db: &mut DBImpl) -> Result<Vec<Self>>;

    /// Sets the tag name.
    async fn set_name(&mut self, db: &mut DBImpl, name: &str) -> Result<()>;

    /// Sets the tag description.
    async fn set_description(&mut self, db: &mut DBImpl, description: &str) -> Result<()>;

    /// Deletes the tag from the database.
    async fn delete(self, db: &mut DBImpl) -> Result<()>;
}

#[async_trait]
impl DBTag for Tag {
    async fn create(db: &mut DBImpl, name: &str, description: &str) -> Result<Self> {
        let id = new_id();

        sqlx::query!(
            "INSERT INTO tag (id, name, description) VALUES (?, ?, ?);",
            id,
            name,
            description
        )
        .execute(&mut *db)
        .await?;

        Self::get(db, &id).await.map(|x| x.unwrap())
    }

    async fn get(db: &mut DBImpl, id: &str) -> Result<Option<Self>> {
        Ok(sqlx::query_as!(Self, "SELECT * FROM tag WHERE id = ?;", id)
            .fetch_optional(&mut *db)
            .await?)
    }

    async fn list(db: &mut DBImpl) -> Result<Vec<Self>> {
        Ok(sqlx::query_as!(Self, "SELECT * FROM tag ORDER BY name;")
            .fetch_all(&mut *db)
            .await?)
    }

    async fn set_name(&mut self, db: &mut DBImpl, name: &str) -> Result<()> {
        self.name = name.to_owned();

        sqlx::query!("UPDATE tag SET name = ? WHERE id = ?;", self.name, self.id)
            .execute(&mut *db)
            .await?;

        Ok(())
    }

    async fn set_description(&mut self, db: &mut DBImpl, description: &str) -> Result<()> {
        self.description = Some(description.to_owned());

        sqlx::query!(
            "UPDATE tag SET description = ? WHERE id = ?;",
            self.description,
            self.id
        )
        .execute(&mut *db)
        .await?;

        Ok(())
    }

    async fn delete(self, db: &mut DBImpl) -> Result<()> {
        sqlx::query!("DELETE FROM tag WHERE id = ?;", self.id)
            .execute(&mut *db)
            .await?;

        Ok(())
    }
}

/// Tag tests.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::TestDB;

    #[tokio::test]
    async fn test_tag() {
        // Init
        let mut db = TestDB::new().await.unwrap();

        // Create
        let mut tag1 = Tag::create(&mut db, "Tag 1", "Tag 1 description")
            .await
            .unwrap();
        let tag2 = Tag::create(&mut db, "Tag 2", "").await.unwrap();

        // Get
        let tag3 = Tag::get(&mut db, &tag1.id).await.unwrap().unwrap();
        assert_eq!(tag3, tag1);
        let tag4 = Tag::get(&mut db, &tag2.id).await.unwrap().unwrap();
        assert_eq!(tag4, tag2);
        assert!(Tag::get(&mut db, "").await.unwrap().is_none());

        // List
        let tags = Tag::list(&mut db).await.unwrap();
        assert_eq!(tags.len(), 2);
        let tag5 = tags.iter().find(|x| x.id == tag1.id).unwrap();
        assert_eq!(tag5, &tag1);
        let tag6 = tags.iter().find(|x| x.id == tag2.id).unwrap();
        assert_eq!(tag6, &tag2);

        // Set name
        tag1.set_name(&mut db, "Not tag 1").await.unwrap();
        assert_eq!(&tag1.name, "Not tag 1");
        let tag7 = Tag::get(&mut db, &tag1.id).await.unwrap().unwrap();
        assert_eq!(tag7, tag1);

        // Set description
        tag1.set_description(&mut db, "Not tag 1 description")
            .await
            .unwrap();
        assert_eq!(
            tag1.description.as_ref().unwrap().as_str(),
            "Not tag 1 description"
        );
        let tag8 = Tag::get(&mut db, &tag1.id).await.unwrap().unwrap();
        assert_eq!(tag8, tag1);

        // Delete
        let tag_id1 = tag1.id.clone();
        assert!(Tag::get(&mut db, &tag_id1).await.unwrap().is_some());
        tag1.delete(&mut db).await.unwrap();
        assert!(Tag::get(&mut db, &tag_id1).await.unwrap().is_none());
        let tag_id2 = tag2.id.clone();
        assert!(Tag::get(&mut db, &tag_id2).await.unwrap().is_some());
        tag2.delete(&mut db).await.unwrap();
        assert!(Tag::get(&mut db, &tag_id2).await.unwrap().is_none());

        // Clean up
        db.delete().await.unwrap();
    }
}
