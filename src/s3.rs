use std::io;

use rusoto_core::RusotoError;
use rusoto_s3::{
    DeleteObjectError, DeleteObjectOutput, DeleteObjectRequest, GetObjectError, GetObjectOutput,
    GetObjectRequest, S3Client, StreamingBody, S3,
};
use tokio::io::AsyncReadExt;

pub async fn get_object(
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

pub async fn read_body(body: StreamingBody, content_length: usize) -> Result<Vec<u8>, io::Error> {
    let mut reader = body.into_async_read();

    let mut data = Vec::with_capacity(content_length);
    reader.read_to_end(&mut data).await?;

    Ok(data)
}

pub async fn delete_object(
    s3_client: &S3Client,
    bucket_name: String,
    object_key: String,
) -> Result<DeleteObjectOutput, RusotoError<DeleteObjectError>> {
    let input = DeleteObjectRequest {
        bucket: bucket_name,
        key: object_key,
        ..Default::default()
    };
    let output = s3_client.delete_object(input).await?;
    Ok(output)
}
