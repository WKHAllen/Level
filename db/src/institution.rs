use crate::{new_id, DBImpl};
use async_trait::async_trait;
use backend_common::Result;
use common::*;

/// The database implementation of the institution model.
#[async_trait]
pub trait DBInstitution: Sized {
    /// Creates a new institution.
    async fn create(db: &mut DBImpl, name: &str, description: &str) -> Result<Self>;

    /// Gets an institution from the database.
    async fn get(db: &mut DBImpl, id: &str) -> Result<Option<Self>>;

    /// Lists all institutions in the database.
    async fn list(db: &mut DBImpl) -> Result<Vec<Self>>;

    /// Sets the institution name.
    async fn set_name(&mut self, db: &mut DBImpl, name: &str) -> Result<()>;

    /// Sets the institution description.
    async fn set_description(&mut self, db: &mut DBImpl, description: &str) -> Result<()>;

    /// Deletes the institution from the database.
    async fn delete(self, db: &mut DBImpl) -> Result<()>;
}

#[async_trait]
impl DBInstitution for Institution {
    async fn create(db: &mut DBImpl, name: &str, description: &str) -> Result<Self> {
        let id = new_id();

        sqlx::query!(
            "INSERT INTO institution (id, name, description) VALUES (?, ?, ?);",
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
            sqlx::query_as!(Self, "SELECT * FROM institution WHERE id = ?;", id)
                .fetch_optional(&mut *db)
                .await?,
        )
    }

    async fn list(db: &mut DBImpl) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM institution ORDER BY name;")
                .fetch_all(&mut *db)
                .await?,
        )
    }

    async fn set_name(&mut self, db: &mut DBImpl, name: &str) -> Result<()> {
        self.name = name.to_owned();

        sqlx::query!(
            "UPDATE institution SET name = ? WHERE id = ?;",
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
            "UPDATE institution SET description = ? WHERE id = ?;",
            self.description,
            self.id
        )
        .execute(&mut *db)
        .await?;

        Ok(())
    }

    async fn delete(self, db: &mut DBImpl) -> Result<()> {
        sqlx::query!("DELETE FROM institution WHERE id = ?;", self.id)
            .execute(&mut *db)
            .await?;

        Ok(())
    }
}

/// Institution tests.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::TestDB;

    #[tokio::test]
    async fn test_institution() {
        // Init
        let mut db = TestDB::new().await.unwrap();

        // Create
        let mut institution1 =
            Institution::create(&mut db, "Institution 1", "Institution 1 description")
                .await
                .unwrap();
        let institution2 = Institution::create(&mut db, "Institution 2", "")
            .await
            .unwrap();

        // Get
        let institution3 = Institution::get(&mut db, &institution1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(institution3, institution1);
        let institution4 = Institution::get(&mut db, &institution2.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(institution4, institution2);
        assert!(Institution::get(&mut db, "").await.unwrap().is_none());

        // List
        let institutions = Institution::list(&mut db).await.unwrap();
        assert_eq!(institutions.len(), 2);
        let institution5 = institutions
            .iter()
            .find(|x| x.id == institution1.id)
            .unwrap();
        assert_eq!(institution5, &institution1);
        let institution6 = institutions
            .iter()
            .find(|x| x.id == institution2.id)
            .unwrap();
        assert_eq!(institution6, &institution2);

        // Set name
        institution1
            .set_name(&mut db, "Not institution 1")
            .await
            .unwrap();
        assert_eq!(&institution1.name, "Not institution 1");
        let institution7 = Institution::get(&mut db, &institution1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(institution7, institution1);

        // Set description
        institution1
            .set_description(&mut db, "Not institution 1 description")
            .await
            .unwrap();
        assert_eq!(
            institution1.description.as_ref().unwrap().as_str(),
            "Not institution 1 description"
        );
        let institution8 = Institution::get(&mut db, &institution1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(institution8, institution1);

        // Delete
        let institution_id1 = institution1.id.clone();
        assert!(Institution::get(&mut db, &institution_id1)
            .await
            .unwrap()
            .is_some());
        institution1.delete(&mut db).await.unwrap();
        assert!(Institution::get(&mut db, &institution_id1)
            .await
            .unwrap()
            .is_none());
        let institution_id2 = institution2.id.clone();
        assert!(Institution::get(&mut db, &institution_id2)
            .await
            .unwrap()
            .is_some());
        institution2.delete(&mut db).await.unwrap();
        assert!(Institution::get(&mut db, &institution_id2)
            .await
            .unwrap()
            .is_none());

        // Clean up
        db.delete().await.unwrap();
    }
}
