use tide::{Request, Response};
use crate::template_models::notes;
use askama::Template;
use tide::http::mime;
use crate::services::notes_service::all_notes;

pub async fn index(_req: Request<()>) -> tide::Result {
    let notes = all_notes().await;
    let notes_template_model = notes::Index {
        notes: &notes
    };
    let body = notes_template_model.render().unwrap();
    let response = Response::builder(200)
        .body(body)
        .content_type(mime::HTML)
        .build();

    Ok(response)
}
