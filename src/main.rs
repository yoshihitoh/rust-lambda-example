use std::error::Error as StdError;
use std::io;

use lambda_runtime::{error::HandlerError, lambda, Context};
use rusoto_core::{Region, RusotoError};
use rusoto_s3::{GetObjectError, GetObjectOutput, GetObjectRequest, S3Client, StreamingBody, S3};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;
use tokio::runtime::Runtime;

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
        .block_on(handle_s3_object(&app, event))
        .map_err(|e| HandlerError::from(format!("error: {:?}", e).as_str()))?;

    Ok(CustomOutput {})
}

async fn handle_s3_object(
    app: &App,
    event: CustomEvent,
) -> Result<(), Box<dyn StdError>> {
    let out = get_object(&app.s3_client, event.bucket_name, event.object_key)
        .await?;

    if let (Some(body), Some(length)) = (out.body, out.content_length) {
        let data = read_body(body, length as usize).await?;
        // オブジェクトのデータを使って何かする
    }

    Ok(())
}

async fn get_object(
    s3_client: &S3Client,
    bucket_name: String,
    object_key: String,
) -> Result<GetObjectOutput, RusotoError<GetObjectError>> {
    let input = GetObjectRequest {
        bucket: bucket_name,
        key: object_key,
        ..Default::default()
    };
    let output = s3_client.get_object(input).await?;
    Ok(output)
}

async fn read_body(
    body: StreamingBody,
    content_length: usize,
) -> Result<Vec<u8>, io::Error> {
    let mut reader = body.into_async_read();

    let mut data = Vec::with_capacity(content_length);
    reader.read_to_end(&mut data).await?;

    Ok(data)
}
