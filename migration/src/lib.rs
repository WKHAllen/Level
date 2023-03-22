#![forbid(unsafe_code)]

pub use migration_macros::*;

pub mod migrations;

/// Test the initial migration.
#[test]
fn test() {
    println!("PERFORMING MIGRATIONS...");
    migrate_from!("");
}
