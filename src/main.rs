use std::error::Error as StdError;

use lambda_runtime::{error::HandlerError, lambda, Context};
use rusoto_core::Region;
use rusoto_s3::{S3Client, S3};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CustomEvent {}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CustomOutput {
    bucket_names: Vec<String>,
}

struct App {
    s3_client: S3Client,
    runtime: Runtime,
}

impl Default for App {
    fn default() -> Self {
        let s3_client = S3Client::new(Region::default());
        let runtime = Runtime::new().unwrap();
        App { s3_client, runtime }
    }
}

fn main() -> Result<(), Box<dyn StdError>> {
    lambda!(handler);
    Ok(())
}

fn handler(_event: CustomEvent, _context: Context) -> Result<CustomOutput, HandlerError> {
    let mut app = App::default();
    let bucket_names = app
        .runtime
        .block_on(bucket_names(&app.s3_client))
        .map_err(|e| HandlerError::from(format!("error: {:?}", e).as_str()))?;
    Ok(CustomOutput { bucket_names })
}

async fn bucket_names(s3_client: &S3Client) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let output = s3_client.list_buckets().await?;
    let names = output
        .buckets
        .map(|bs| {
            bs.into_iter()
                .filter(|b| b.name.is_some())
                .map(|b| b.name.unwrap())
                .collect()
        })
        .unwrap_or_else(Vec::new);

    Ok(names)
}
