// use ntex::web::{self, HttpResponse};
// use serde::Deserialize;

// use crate::github::push_to_github;

// #[derive(Deserialize)]
// pub struct UploadRequest {
//     filename: String,
//     content: String,
// }

// pub async fn handle_upload(req: web::types::Json<UploadRequest>) -> HttpResponse {
//     let path = format!("src/content/blog/{}.md", req.filename);
//     let content = &req.content;

//     match push_to_github(&path, content).await {
//         Ok(_) => HttpResponse::Ok().body("Uploaded to GitHub"),
//         Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
//     }
// }
