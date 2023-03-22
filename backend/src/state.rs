use anyhow::Result;
use backend_macros::backend_commands;
use commands::BackendCommands;
use db::{Quote, DB};
use rand::prelude::*;

/// The backend application state.
pub struct State {
    /// The backend database.
    pub(crate) db: DB,
}

impl State {
    /// Initialize the backend state and connect to the test database.
    pub async fn new() -> Result<Self> {
        let db = DB::open("test").await?;

        Ok(Self { db })
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
        let quotes = Quote::list(&self.db).await;

        if quotes.len() > 0 {
            let index = random::<usize>() % quotes.len();
            quotes[index].quote.clone()
        } else {
            "No quotes found".to_owned()
        }
    }
}
