#![forbid(unsafe_code)]

use macros::command_trait;

/// Global application commands, designed to facilitate communication between the frontend and backend.
#[command_trait]
pub trait Commands {
    /// Prints "Hi!" to stdout.
    async fn say_hi(&self);

    /// Greets a person by name.
    async fn greet(&self, name: String) -> String;

    /// Retrieves a random quote from the database.
    async fn get_random_quote(&self) -> String;
}
