use crate::{new_id, DB};
use backend_common::Result;
use chrono::NaiveDateTime;
use serde::{de::DeserializeOwned, ser::Serialize};

/// A representation of a report template in the database.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReportTemplate {
    /// The report template's identifier.
    pub id: String,
    /// The name of the report template.
    pub name: String,
    /// A description of the report template.
    pub description: Option<String>,
    /// The dynamic report template structure, serialized as a String.
    pub data: String,
    /// When the report template was created.
    pub created_at: NaiveDateTime,
}

impl ReportTemplate {
    /// Creates a new report template.
    pub async fn create(db: &mut DB, name: &str, description: &str) -> Result<Self> {
        let id = new_id();

        sqlx::query!(
            "INSERT INTO report_template (id, name, description, data) VALUES (?, ?, ?, ?);",
            id,
            name,
            description,
            "{}"
        )
        .execute(&mut **db)
        .await?;

        Self::get(db, &id).await.map(|x| x.unwrap())
    }

    /// Gets a report template from the database.
    pub async fn get(db: &mut DB, id: &str) -> Result<Option<Self>> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM report_template WHERE id = ?;", id)
                .fetch_optional(&mut **db)
                .await?,
        )
    }

    /// Lists all report templates in the database.
    pub async fn list(db: &mut DB) -> Result<Vec<Self>> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM report_template ORDER BY name;")
                .fetch_all(&mut **db)
                .await?,
        )
    }

    /// Gets the deserialized template data. This can fail if deserialization fails.
    pub fn get_data<T: DeserializeOwned>(&self) -> Result<T> {
        Ok(serde_json::from_str(&self.data)?)
    }

    /// Sets the report template name.
    pub async fn set_name(&mut self, db: &mut DB, name: &str) -> Result<()> {
        self.name = name.to_owned();

        sqlx::query!(
            "UPDATE report_template SET name = ? WHERE id = ?;",
            self.name,
            self.id
        )
        .execute(&mut **db)
        .await?;

        Ok(())
    }

    /// Sets the report template description.
    pub async fn set_description(&mut self, db: &mut DB, description: &str) -> Result<()> {
        self.description = Some(description.to_owned());

        sqlx::query!(
            "UPDATE report_template SET description = ? WHERE id = ?;",
            self.description,
            self.id
        )
        .execute(&mut **db)
        .await?;

        Ok(())
    }

    /// Serializes and sets the template data. This can fail if serialization fails.
    pub async fn set_data<T: Serialize + ?Sized>(&mut self, db: &mut DB, data: &T) -> Result<()> {
        self.data = serde_json::to_string(&data)?;

        sqlx::query!(
            "UPDATE report_template SET data = ? WHERE id = ?;",
            self.data,
            self.id
        )
        .execute(&mut **db)
        .await?;

        Ok(())
    }

    /// Deletes the template from the database.
    pub async fn delete(self, db: &mut DB) -> Result<()> {
        sqlx::query!("DELETE FROM report_template WHERE id = ?;", self.id)
            .execute(&mut **db)
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
