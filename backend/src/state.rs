use anyhow::Result;
use backend_macros::backend_commands;
use commands::BackendCommands;
use db::Save;
use std::sync::Arc;
use tokio::sync::Mutex;

/// The backend application state.
pub struct State {
    /// The backend database.
    save: Arc<Mutex<Save>>,
}

impl State {
    /// Initialize the backend state and connect to the test database.
    pub async fn new() -> Result<Self> {
        let save_name = "test";
        let save_description = "A test save.";
        let save_password = "password123";

        let save = if Save::exists(save_name) {
            Save::open(save_name, save_password).await?
        } else {
            Save::create(save_name, save_description, save_password).await?
        };

        Ok(Self {
            save: Arc::new(Mutex::new(save)),
        })
    }
}

#[backend_commands]
impl BackendCommands for State {
    async fn say_hi(&self) {
        println!("Hi!");
    }

    async fn greet(&self, name: String) -> String {
        format!("Hello, {}!", name)
    }

    async fn get_random_quote(&self) -> String {
        "Quotes demo removed".to_owned()
    }
}
