use std::{collections::HashMap, path::Path};

use futures::{Stream, StreamExt};
use sea_orm::DatabaseConnection;
use tokio::{fs::File, io::AsyncWriteExt};
use warp::{hyper::StatusCode, multipart::FormData, Buf, Filter, Rejection};

use crate::auth::Session;

use super::{error::Error, middlewares::extract_session};

pub fn create_content(
    content_dir: &str,
    db: &DatabaseConnection,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let content_dir = content_dir.to_owned();
    let db = db.clone();
    let serve = warp::fs::dir(content_dir.clone());
    let upload = warp::path::end()
        .and(warp::post())
        .and(extract_session(&db))
        .and(warp::multipart::form())
        .and_then(move |session: Option<Session>, mut form: FormData| {
            let content_dir = content_dir.clone();
            async move {
                let user = session
                    .ok_or(Error::with_code_and_message(
                        StatusCode::UNAUTHORIZED,
                        "UNAUTHORIZED",
                    ))?
                    .user;
                let mut files = HashMap::new();
                while let Some(Ok(part)) = form.next().await {
                    if let Some(filename) = part.filename() {
                        let relative_path = Path::new(&user.id.to_string()).join(filename);
                        let path = Path::new(&content_dir).join(&relative_path);
                        files.insert(
                            filename.to_owned(),
                            relative_path
                                .to_str()
                                .ok_or(Error::with_message(&format!(
                                    "path of file {} includes non utf-8 characters",
                                    relative_path.to_string_lossy()
                                )))?
                                .to_owned(),
                        );
                        let stream = part.stream();
                        write_stream_to_file(path, stream)
                            .await
                            .map_err(|e| Error::with_message(&e.to_string()))?;
                    }
                }
                Ok::<_, Rejection>(warp::reply::json(&files))
            }
        });

    upload.or(serve)
}

async fn write_stream_to_file(
    filepath: impl AsRef<Path>,
    stream: impl Stream<Item = Result<impl Buf, warp::Error>>,
) -> Result<(), std::io::Error> {
    let mut file = File::create(filepath).await?;
    let mut stream = Box::pin(stream);
    while let Some(Ok(buf)) = stream.next().await {
        file.write_all(buf.chunk()).await?;
    }
    file.sync_all().await?;
    Ok(())
}
