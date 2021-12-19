use std::{fs, io::Write};

use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use itertools::Itertools;
use sqlx::PgPool;

use crate::{extractors::AuthorizationService, io::error::Error};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UploadPostPhotoResponse {
    filepath: Option<String>,
}

pub async fn upload_post_photo(
    _auth_service: AuthorizationService,
    _conn: web::Data<PgPool>,
    payload: Multipart,
) -> anyhow::Result<HttpResponse, Error> {
    let filepath = save_file(payload).await.map_err(|e| {
        eprintln!("{:?}", e);
        Error::InternalServerError
    })?;

    match filepath {
        Some(f) => Ok(HttpResponse::Ok().json(UploadPostPhotoResponse { filepath: Some(f) })),
        None => Ok(HttpResponse::Ok().json(UploadPostPhotoResponse { filepath: None })),
    }
}

async fn save_file(mut payload: Multipart) -> Result<Option<String>, anyhow::Error> {
    let mut filepath_response: Option<String> = None;

    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let fieldname = content_type.get_name().unwrap();

        match fieldname {
            "file" => {
                let filename = content_type.get_filename().unwrap();

                let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename))
                    .to_lowercase()
                    .split(" ")
                    .join("-");

                filepath_response = Some(filepath.clone());

                // File::create is blocking operation, use threadpool
                let mut f = web::block(|| std::fs::File::create(filepath)).await??;
                // Field in turn is stream of *Bytes* object
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    // filesystem operations are blocking, we have to use threadpool
                    f = web::block(move || f.write_all(&data).map(|_| f)).await??;
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
