use crate::convert_file_name;
use crate::db::*;
use backend_common::*;
use chrono::{NaiveDateTime, Utc};
use common::*;
use crypto::*;
use std::collections::HashMap;
use std::io;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use tokio::fs::{self, File};

/// The directory in which database save files are stored.
pub(crate) const SAVES_DIR: &str = "saves";

/// The file extension used to identify save files.
pub(crate) const SAVE_EXT: &str = "level";

/// The file extension used to identify temporary save files.
pub(crate) const TMP_SAVE_EXT: &str = "tmp";

/// Creates the saves directory if it does not already exist.
pub(crate) async fn init_saves_dir() -> io::Result<()> {
    let root_path = project_root::get_project_root()?;
    let saves_dir = root_path.join(SAVES_DIR);

    if !saves_dir.exists() {
        fs::create_dir(&saves_dir).await?;
    }

    Ok(())
}

/// Gets the path to the saves directory.
pub(crate) fn get_saves_path() -> String {
    let root_path = project_root::get_project_root().unwrap();
    let saves_path = format!("{}/{}", root_path.display(), SAVES_DIR);
    saves_path
}

/// Gets the path to a database save file.
pub(crate) fn get_save_path(name: &str) -> String {
    let file_name = convert_file_name(name);
    let root_path = project_root::get_project_root().unwrap();
    let save_path = format!(
        "{}/{}/{}.{}",
        root_path.display(),
        SAVES_DIR,
        file_name,
        SAVE_EXT
    );
    save_path
}

/// Gets the path to a temporary database save file.
pub(crate) fn get_tmp_save_path(name: &str) -> String {
    format!("{}.{}", get_save_path(name), TMP_SAVE_EXT)
}

/// Reads the metadata section of a save file.
async fn read_save_metadata(save_file: &mut File) -> io::Result<String> {
    match read_section(save_file).await? {
        Some(data) => {
            String::from_utf8(data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        }
        None => Ok("".to_owned()),
    }
}

/// Skips the metadata section of a save file.
async fn skip_metadata(save_file: &mut File) -> io::Result<()> {
    read_section(save_file).await?;

    Ok(())
}

/// A metadata parsing trait.
trait Metadata {
    /// Parse the metadata properties, using default values when necessary.
    fn read(save_name: &str, metadata: &str) -> Self;

    /// Output the metadata properties in save file format.
    fn write(&self) -> String;
}

impl Metadata for SaveMetadata {
    fn read(save_name: &str, metadata: &str) -> Self {
        let metadata_pairs = metadata
            .split('\n')
            .filter_map(|line| {
                line.split_once('=')
                    .map(|(key, value)| (key.to_owned(), value.to_owned()))
            })
            .collect::<HashMap<String, String>>();

        let name = metadata_pairs
            .get("name")
            .unwrap_or(&save_name.to_owned())
            .to_owned();
        let description = metadata_pairs
            .get("description")
            .unwrap_or(&String::new())
            .to_owned();
        let created_at = NaiveDateTime::from_timestamp_opt(
            metadata_pairs
                .get("created_at")
                .unwrap_or(&String::new())
                .parse()
                .unwrap_or(Utc::now().timestamp()),
            0,
        )
        .unwrap_or(Utc::now().naive_utc());
        let last_opened_at = NaiveDateTime::from_timestamp_opt(
            metadata_pairs
                .get("last_opened_at")
                .unwrap_or(&String::new())
                .parse()
                .unwrap_or(Utc::now().timestamp()),
            0,
        )
        .unwrap_or(Utc::now().naive_utc());

        Self {
            name,
            description,
            created_at,
            last_opened_at,
        }
    }

    fn write(&self) -> String {
        let name = format!("name={}", self.name);
        let description = format!("description={}", self.description);
        let created_at = format!("created_at={}", self.created_at.timestamp());
        let last_opened_at = format!("last_opened_at={}", self.last_opened_at.timestamp());

        [name, description, created_at, last_opened_at].join("\n")
    }
}

/// An encrypted database save file.
#[derive(Debug)]
pub struct Save {
    /// The decrypted database.
    db: DB,
    /// The encryption key.
    key: [u8; AES_KEY_SIZE],
    /// The save file's metadata.
    metadata: SaveMetadata,
}

impl Save {
    /// Creates a new save file.
    pub async fn create(name: &str, description: &str, password: &str) -> Result<Self> {
        init_saves_dir().await?;

        Self::verify_does_not_exist(name)?;

        let key = password_to_key(password).await;
        let now = Utc::now().naive_utc();

        let metadata = SaveMetadata {
            name: name.to_owned(),
            description: description.to_owned(),
            created_at: now,
            last_opened_at: now,
        };

        let db = DB::create(name).await?;
        let mut this = Self { db, key, metadata };
        this.save().await?;

        Ok(this)
    }

    /// Opens and decrypts a save file.
    pub async fn open(name: &str, password: &str) -> Result<Self> {
        init_saves_dir().await?;

        Self::verify_exists(name)?;

        let key = password_to_key(password).await;

        let save_path = get_save_path(name);
        let mut save_file = File::open(&save_path).await?;
        let metadata_str = read_save_metadata(&mut save_file).await?;
        let mut metadata = SaveMetadata::read(name, &metadata_str);

        let maybe_db = DB::create_with(name, move |mut db_file| async move {
            decrypt_file(&mut save_file, &mut db_file, &key).await
        })
        .await;

        let db = match maybe_db {
            Ok(db) => Ok(db),
            Err(e) => {
                fs::remove_file(get_db_path(name)).await?;
                Err(e)
            }
        }?;

        metadata.last_opened_at = Utc::now().naive_utc();
        Self::save_metadata(name, &metadata).await?;

        Ok(Self { db, key, metadata })
    }

    /// Saves the state of the database to the save file.
    pub async fn save(&mut self) -> Result<()> {
        let save_path = get_save_path(&self.metadata.name);
        let tmp_save_path = get_tmp_save_path(&self.metadata.name);

        {
            let mut tmp_save_file = File::create(&tmp_save_path).await?;
            let metadata_str = self.metadata.write();
            write_section(&mut tmp_save_file, metadata_str.as_bytes()).await?;

            let key = self.key;

            self.db
                .pause_with(move |mut db_file| async move {
                    encrypt_file(&mut db_file, &mut tmp_save_file, &key).await
                })
                .await?;
        }

        fs::rename(tmp_save_path, save_path).await?;

        Ok(())
    }

    /// Saves and closes the database save file.
    pub async fn close(self) -> Result<()> {
        let save_path = get_save_path(&self.metadata.name);
        let tmp_save_path = get_tmp_save_path(&self.metadata.name);

        {
            let mut tmp_save_file = File::create(&tmp_save_path).await?;
            let metadata_str = self.metadata.write();
            write_section(&mut tmp_save_file, metadata_str.as_bytes()).await?;

            let key = self.key;

            self.db
                .delete_with(move |mut db_file| async move {
                    encrypt_file(&mut db_file, &mut tmp_save_file, &key).await
                })
                .await?;
        }

        fs::rename(tmp_save_path, save_path).await?;

        Ok(())
    }

    /// Checks if a save with the given name exists.
    pub fn exists(name: &str) -> bool {
        Path::new(&get_save_path(name)).exists()
    }

    /// Gets the metadata of the save file without needing to decrypt the
    /// file.
    pub async fn metadata(name: &str) -> Result<SaveMetadata> {
        let save_path = get_save_path(name);
        let mut save_file = File::open(&save_path).await?;
        let metadata_str = read_save_metadata(&mut save_file).await?;
        let metadata = SaveMetadata::read(name, &metadata_str);

        Ok(metadata)
    }

    /// Gets the metadata of the currently open save file.
    pub fn this_metadata(&self) -> SaveMetadata {
        self.metadata.clone()
    }

    /// Save a save file's metadata.
    async fn save_metadata(name: &str, metadata: &SaveMetadata) -> Result<()> {
        let save_path = get_save_path(name);
        let tmp_save_path = get_tmp_save_path(name);

        {
            let mut save_file = File::open(&save_path).await?;
            skip_metadata(&mut save_file).await?;

            let mut tmp_save_file = File::create(&tmp_save_path).await?;
            let metadata_str = metadata.write();
            write_section(&mut tmp_save_file, metadata_str.as_bytes()).await?;
            copy_file_in_chunks(&mut save_file, &mut tmp_save_file).await?;
        }

        fs::rename(tmp_save_path, save_path).await?;

        Ok(())
    }

    /// Checks if a save with the provided name exists, returning an error if
    /// it does not.
    fn verify_exists(name: &str) -> Result<()> {
        if Self::exists(name) {
            Ok(())
        } else {
            Err(ExpectedCommandError::SaveNotFound)?
        }
    }

    /// Checks if a save with the provided name exists, returning an error if
    /// it does.
    fn verify_does_not_exist(name: &str) -> Result<()> {
        if Self::exists(name) {
            Err(ExpectedCommandError::SaveAlreadyExists)?
        } else {
            Ok(())
        }
    }

    /// Checks if the provided password can successfully decrypt a save,
    /// returning an error if it cannot.
    async fn verify_password(name: &str, password: &str) -> Result<()> {
        let key = password_to_key(password).await;

        let save_path = get_save_path(name);
        let mut save_file = File::open(&save_path).await?;
        skip_metadata(&mut save_file).await?;

        try_decrypt_file(&mut save_file, &key).await?;

        Ok(())
    }

    /// Sets the name of a save. This should not be used while the save is
    /// open.
    pub async fn set_name(old_name: &str, new_name: &str, password: &str) -> Result<()> {
        Self::verify_password(old_name, password).await?;
        Self::verify_does_not_exist(new_name)?;

        let mut metadata = Self::metadata(old_name).await?;
        metadata.name = new_name.to_owned();

        Self::save_metadata(old_name, &metadata).await?;

        let old_path = get_save_path(old_name);
        let new_path = get_save_path(new_name);
        fs::rename(old_path, new_path).await?;

        Ok(())
    }

    /// Sets the description of a save. This should not be used while the save is open.
    pub async fn set_description(name: &str, description: &str, password: &str) -> Result<()> {
        Self::verify_password(name, password).await?;

        let mut metadata = Self::metadata(name).await?;
        metadata.description = description.to_owned();

        Self::save_metadata(name, &metadata).await?;

        Ok(())
    }

    /// Changes a save's password by decrypting and re-encrypting the save
    /// data. This should not be used while the save is open.
    pub async fn change_password(name: &str, old_password: &str, new_password: &str) -> Result<()> {
        let new_key = password_to_key(new_password).await;

        let mut save = Self::open(name, old_password).await?;
        save.key = new_key;
        save.close().await
    }

    /// Deletes a save. This should not be used while the save is open.
    pub async fn delete(name: &str, password: &str) -> Result<()> {
        Self::verify_password(name, password).await?;

        let save_path = get_save_path(name);

        fs::remove_file(save_path).await?;

        Ok(())
    }

    /// Lists metadata on all saves.
    pub async fn list() -> Result<Vec<SaveMetadata>> {
        init_saves_dir().await?;

        let saves_path = get_saves_path();
        let mut files = fs::read_dir(&saves_path).await?;

        let mut saves = Vec::new();

        while let Some(file) = files.next_entry().await? {
            if let Some(file_name) = file.file_name().to_str() {
                if let Some(file_ext) = Path::new(file_name).extension() {
                    if file_ext == SAVE_EXT {
                        if let Some((name, _)) = file_name.rsplit_once('.') {
                            let metadata = Self::metadata(name).await?;
                            saves.push(metadata);
                        }
                    }
                }
            }
        }

        Ok(saves)
    }
}

impl Deref for Save {
    type Target = DB;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}

impl DerefMut for Save {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.db
    }
}

/// Save file tests.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::DBTag;
    use common::Tag;

    const TEST_SAVE_NAMES: &[&str] = &["test", "My Test Save"];

    #[tokio::test]
    async fn test_save() {
        let name = "Test save name";
        let description = "Test save description";
        let password = "password123";

        // Create/use/close
        let mut save = Save::create(name, description, password).await.unwrap();
        let tag1 = Tag::create(&mut save, "Test tag", "").await.unwrap();
        save.close().await.unwrap();

        // Get metadata
        let metadata = Save::metadata(name).await.unwrap();
        assert_eq!(&metadata.name, name);
        assert_eq!(&metadata.description, description);
        assert_eq!(metadata.created_at, metadata.last_opened_at);

        // Open/use
        let mut save = Save::open(name, password).await.unwrap();
        let tag2 = Tag::get(&mut save, &tag1.id).await.unwrap().unwrap();
        assert_eq!(tag1, tag2);

        // Save/close
        save.save().await.unwrap();
        save.close().await.unwrap();

        // Check existence
        let exists = Save::exists(name);
        assert!(exists);
        let exists = Save::exists("wrong name");
        assert!(!exists);

        // Wrong password
        let failed_save = Save::open(name, "wrong password").await;
        assert!(failed_save.is_err());

        // Set name
        let new_name = "New name";
        Save::set_name(name, new_name, password).await.unwrap();
        let failed_save = Save::open(name, password).await;
        assert!(failed_save.is_err());
        let save = Save::open(new_name, password).await.unwrap();
        save.close().await.unwrap();
        let metadata = Save::metadata(new_name).await.unwrap();
        assert_eq!(&metadata.name, new_name);

        // Set description
        let new_description = "New description";
        Save::set_description(new_name, new_description, password)
            .await
            .unwrap();
        let metadata = Save::metadata(new_name).await.unwrap();
        assert_eq!(&metadata.description, new_description);

        // Change password
        let new_password = "New password";
        Save::change_password(new_name, password, new_password)
            .await
            .unwrap();
        let failed_save = Save::open(new_name, password).await;
        assert!(failed_save.is_err());
        let save = Save::open(new_name, new_password).await.unwrap();
        save.close().await.unwrap();

        // List
        let saves = Save::list().await.unwrap();
        let saves = saves
            .into_iter()
            .filter(|s| !TEST_SAVE_NAMES.contains(&s.name.as_str())) // ignore the test saves
            .collect::<Vec<_>>();
        assert_eq!(saves.len(), 1);
        assert_eq!(&saves[0].name, new_name);
        let name2 = "Other save";
        let description2 = "Another save";
        let password2 = "A password even worse than 'password123'";
        let other_save = Save::create(name2, description2, password2).await.unwrap();
        assert_eq!(&other_save.metadata.name, name2);
        other_save.close().await.unwrap();
        let saves = Save::list().await.unwrap();
        let saves = saves
            .into_iter()
            .filter(|s| !TEST_SAVE_NAMES.contains(&s.name.as_str())) // ignore the test saves
            .collect::<Vec<_>>();
        assert_eq!(saves.len(), 2);
        let save1 = saves.iter().find(|s| s.name == new_name).unwrap();
        let save2 = saves.iter().find(|s| s.name == name2).unwrap();
        assert_eq!(&save1.description, new_description);
        assert_eq!(&save2.description, description2);

        // Delete
        Save::delete(new_name, new_password).await.unwrap();
        let saves = Save::list().await.unwrap();
        let saves = saves
            .into_iter()
            .filter(|s| !TEST_SAVE_NAMES.contains(&s.name.as_str())) // ignore the test saves
            .collect::<Vec<_>>();
        assert_eq!(saves.len(), 1);
        Save::delete(name2, password2).await.unwrap();
        let saves = Save::list().await.unwrap();
        let saves = saves
            .into_iter()
            .filter(|s| !TEST_SAVE_NAMES.contains(&s.name.as_str())) // ignore the test saves
            .collect::<Vec<_>>();
        assert!(saves.is_empty());
    }
}
