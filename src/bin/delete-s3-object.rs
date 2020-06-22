use std::error::Error as StdError;

use lambda_runtime::{error::HandlerError, lambda, Context};
use rusoto_core::Region;
use rusoto_s3::S3Client;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

use rust_lambda_example::s3::delete_object;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CustomEvent {
    bucket_name: String,
    object_key: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CustomOutput {}

struct App {
    s3_client: S3Client,
}

impl Default for App {
    fn default() -> Self {
        let s3_client = S3Client::new(Region::default());
        App { s3_client }
    }
}

fn main() -> Result<(), Box<dyn StdError>> {
    let app = App::default();
    let mut runtime = Runtime::new().expect("Cannot create a tokio runtime.");

    lambda!(|event, context| handler(&mut runtime, &app, event, context));

    Ok(())
}

fn handler(
    runtime: &mut Runtime,
    app: &App,
    event: CustomEvent,
    _context: Context,
) -> Result<CustomOutput, HandlerError> {
    runtime
        .block_on(delete_s3_object(&app, event))
        .map_err(|e| HandlerError::from(format!("error: {:?}", e).as_str()))?;

    Ok(CustomOutput {})
}

async fn delete_s3_object(app: &App, event: CustomEvent) -> Result<(), Box<dyn StdError>> {
    let _out = delete_object(&app.s3_client, event.bucket_name, event.object_key).await?;

    Ok(())
}
