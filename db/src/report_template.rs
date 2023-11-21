use crate::{new_id, DBImpl};
use async_trait::async_trait;
use backend_common::Result;
use common::*;
use serde::ser::Serialize;

/// The database implementation of the report template model.
#[async_trait]
pub trait DBReportTemplate: Sized {
    /// Creates a new report template.
    async fn create(db: &mut DBImpl, name: &str, description: &str) -> Result<Self>;

    /// Gets a report template from the database.
    async fn get(db: &mut DBImpl, id: &str) -> Result<Option<Self>>;

    /// Lists all report templates in the database.
    async fn list(db: &mut DBImpl) -> Result<Vec<Self>>;

    /// Sets the report template name.
    async fn set_name(&mut self, db: &mut DBImpl, name: &str) -> Result<()>;

    /// Sets the report template description.
    async fn set_description(&mut self, db: &mut DBImpl, description: &str) -> Result<()>;

    /// Serializes and sets the template data. This can fail if serialization fails.
    async fn set_data<T>(&mut self, db: &mut DBImpl, data: &T) -> Result<()>
    where
        T: Serialize + Sync + ?Sized;

    /// Deletes the template from the database.
    async fn delete(self, db: &mut DBImpl) -> Result<()>;
}

#[async_trait]
impl DBReportTemplate for ReportTemplate {
    async fn create(db: &mut DBImpl, name: &str, description: &str) -> Result<Self> {
        let id = new_id();

        sqlx::query!(
            "INSERT INTO report_template (id, name, description, data) VALUES (?, ?, ?, ?);",
            id,
            name,
            description,
            "{}"
        )
        .execute(&mut *db)
        .await?;

        Self::get(db, &id).await.map(|x| x.unwrap())
    }

    async fn get(db: &mut DBImpl, id: &str) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM report_template WHERE id = ?;", id)
                .fetch_optional(&mut *db)
                .await?,
        )
    }

    async fn list(db: &mut DBImpl) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM report_template ORDER BY name;")
                .fetch_all(&mut *db)
                .await?,
        )
    }

    async fn set_name(&mut self, db: &mut DBImpl, name: &str) -> Result<()> {
        self.name = name.to_owned();

        sqlx::query!(
            "UPDATE report_template SET name = ? WHERE id = ?;",
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
            "UPDATE report_template SET description = ? WHERE id = ?;",
            self.description,
            self.id
        )
        .execute(&mut *db)
        .await?;

        Ok(())
    }

    async fn set_data<T>(&mut self, db: &mut DBImpl, data: &T) -> Result<()>
    where
        T: Serialize + Sync + ?Sized,
    {
        self.data = serde_json::to_string(&data)?;

        sqlx::query!(
            "UPDATE report_template SET data = ? WHERE id = ?;",
            self.data,
            self.id
        )
        .execute(&mut *db)
        .await?;

        Ok(())
    }

    async fn delete(self, db: &mut DBImpl) -> Result<()> {
        sqlx::query!("DELETE FROM report_template WHERE id = ?;", self.id)
            .execute(&mut *db)
            .await?;

        Ok(())
    }
}

/// Report template tests.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::TestDB;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct EmptyTemplate {}

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct TestTemplate {
        pub name: String,
        pub age: u8,
        pub rustacean: bool,
    }

    #[tokio::test]
    async fn test_report_template() {
        // Init
        let mut db = TestDB::new().await.unwrap();

        // Create
        let mut template1 = ReportTemplate::create(&mut db, "Template 1", "First template")
            .await
            .unwrap();
        let mut template2 = ReportTemplate::create(&mut db, "Template 2", "Second template")
            .await
            .unwrap();

        // Get
        let template3 = ReportTemplate::get(&mut db, &template1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(template3, template1);
        let template4 = ReportTemplate::get(&mut db, &template2.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(template4, template2);
        assert!(ReportTemplate::get(&mut db, "").await.unwrap().is_none());

        // List
        let templates = ReportTemplate::list(&mut db).await.unwrap();
        assert_eq!(templates.len(), 2);
        let template5 = templates.iter().find(|x| x.id == template1.id).unwrap();
        assert_eq!(template5, &template1);
        let template6 = templates.iter().find(|x| x.id == template2.id).unwrap();
        assert_eq!(template6, &template2);

        // Get data
        let data1: EmptyTemplate = template1.get_data().unwrap();
        let data2: EmptyTemplate = template2.get_data().unwrap();
        assert_eq!(data1, data2);
        assert!(template1.get_data::<TestTemplate>().is_err());
        assert!(template2.get_data::<TestTemplate>().is_err());

        // Set name
        template1.set_name(&mut db, "Not template 1").await.unwrap();
        assert_eq!(&template1.name, "Not template 1");
        let template7 = ReportTemplate::get(&mut db, &template1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(template7, template1);

        // Set description
        template1
            .set_description(&mut db, "Not template 1 description")
            .await
            .unwrap();
        assert_eq!(
            template1.description.as_ref().unwrap().as_str(),
            "Not template 1 description"
        );
        let template8 = ReportTemplate::get(&mut db, &template1.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(template8, template1);

        // Set data
        let data3 = TestTemplate {
            name: "Will".to_owned(),
            age: 24,
            rustacean: true,
        };
        template1.set_data(&mut db, &data3).await.unwrap();
        let data4: TestTemplate = template1.get_data().unwrap();
        assert_eq!(data4, data3);
        let data5 = EmptyTemplate {};
        template2.set_data(&mut db, &data5).await.unwrap();
        let data6: EmptyTemplate = template2.get_data().unwrap();
        assert_eq!(data6, data5);
        assert_eq!(&template2.data, "{}");
        assert!(template2.get_data::<TestTemplate>().is_err());

        // Delete
        let template_id1 = template1.id.clone();
        assert!(ReportTemplate::get(&mut db, &template_id1)
            .await
            .unwrap()
            .is_some());
        template1.delete(&mut db).await.unwrap();
        assert!(ReportTemplate::get(&mut db, &template_id1)
            .await
            .unwrap()
            .is_none());
        let template_id2 = template2.id.clone();
        assert!(ReportTemplate::get(&mut db, &template_id2)
            .await
            .unwrap()
            .is_some());
        template2.delete(&mut db).await.unwrap();
        assert!(ReportTemplate::get(&mut db, &template_id2)
            .await
            .unwrap()
            .is_none());

        // Clean up
        db.delete().await.unwrap();
    }
}
