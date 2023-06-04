use rand::prelude::*;

/// Generates a random 64-bit identifier as a hexadecimal string.
pub fn new_id() -> String {
    let value: u64 = random();
    let hex_value = format!("{value:x}");
    hex_value
}
