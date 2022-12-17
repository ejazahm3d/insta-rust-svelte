use std::io::Write;

use axum::{extract::Multipart, response::IntoResponse, Json};
use futures::StreamExt;
use itertools::Itertools;

use crate::{extractors::AuthUser, io::error::AppError};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UploadPostPhotoResponse {
    filepath: Option<String>,
}

pub async fn upload_post_photo(
    _auth_service: AuthUser,
    payload: Multipart,
) -> anyhow::Result<impl IntoResponse, AppError> {
    let filepath = save_file(payload).await.map_err(|e| {
        eprintln!("{:?}", e);
        AppError::InternalServerError
    })?;

    match filepath {
        Some(f) => Ok(Json(UploadPostPhotoResponse { filepath: Some(f) })),
        None => Ok(Json(UploadPostPhotoResponse { filepath: None })),
    }
}

async fn save_file(mut payload: Multipart) -> Result<Option<String>, anyhow::Error> {
    let mut filepath_response: Option<String> = None;

    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.next_field().await {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "file" => {
                let file_name = field.file_name().unwrap().to_string();

                let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&file_name))
                    .to_lowercase()
                    .split(" ")
                    .join("-");

                filepath_response = Some(filepath.clone());

                // File::create is blocking operation, use threadpool
                let mut f =
                    tokio::task::spawn_blocking(|| std::fs::File::create(filepath)).await??;
                // Field in turn is stream of *Bytes* object
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    // filesystem operations are blocking, we have to use threadpool
                    f = tokio::task::spawn_blocking(move || f.write_all(&data).map(|_| f))
                        .await??;
                }
            }
            _ => println!("not received with 'file'"),
        }
    }

    match filepath_response {
        Some(f) => Ok(Some(f.split("").skip(2).join(""))),
        None => Ok(None),
    }
}
