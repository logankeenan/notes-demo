use tide::{Request, Response};
use crate::template_models::notes;
use askama::Template;
use tide::http::mime;

pub async fn index(_req: Request<()>) -> tide::Result {
    let notes_template_model = notes::Index {};
    let body = notes_template_model.render().unwrap();

    let response = Response::builder(200)
        .body(body)
        .content_type(mime::HTML)
        .build();

    Ok(response)
}
