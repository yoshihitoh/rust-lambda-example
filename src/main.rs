use std::error::Error as StdError;

use lambda_runtime::{error::HandlerError, lambda, Context};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CustomEvent {
    first_name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CustomOutput {
    message: String,
}

fn main() -> Result<(), Box<dyn StdError>> {
    lambda!(handler);
    Ok(())
}

fn handler(event: CustomEvent, _context: Context) -> Result<CustomOutput, HandlerError> {
    Ok(CustomOutput {
        message: format!("Hello, {}!", event.first_name),
    })
}
