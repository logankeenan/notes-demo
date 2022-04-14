use tide::{Request, Response, ResponseBuilder};
use crate::template_models::notes;
use askama::Template;
use pulldown_cmark::{Parser, Options, html};
use surf::{RequestBuilder, Response as SurfResponse};
use tide::http::mime;
use crate::models::note::{NoteForm, Note};
use crate::AppState;
use crate::template_models::notes::{Edit, New, Show};

fn api_url_for_path(state: &AppState, path: &str) -> String {
    let api_url = state.environment.get("API_ORIGIN").unwrap();

    format!("{}{}", api_url, path)
}

fn apply_user_to_request(tide_request: &Request<AppState>, request_builder: RequestBuilder) -> RequestBuilder {
    match tide_request.header("cookie") {
        None => { request_builder }
        Some(cookie_value) => {
            request_builder.header("cookie", cookie_value)
        }
    }
}

fn surf_get_request(url: String, tide_request: &Request<AppState>) -> RequestBuilder {
    apply_user_to_request(tide_request, surf::get(url))
}

fn surf_post_request(url: String, tide_request: &Request<AppState>) -> RequestBuilder {
    apply_user_to_request(tide_request, surf::post(url))
}

fn surf_put_request(url: String, tide_request: &Request<AppState>) -> RequestBuilder {
    apply_user_to_request(tide_request, surf::put(url))
}

fn tide_response(status_code: u16, surf_response: SurfResponse) -> ResponseBuilder {
    let response = Response::builder(status_code);
    match surf_response.header("set-cookie") {
        None => {
            response
        }
        Some(cookie_value) => {
            response.header("set-cookie", cookie_value)
        }
    }
}

pub async fn index(tide_request: Request<AppState>) -> tide::Result {
    let api_request: RequestBuilder = surf_get_request(
        api_url_for_path(tide_request.state(), "/notes"),
        &tide_request,
    );
    let mut api_response = api_request.await?;
    let note: Vec<Note> = api_response.body_json().await?;

    let notes_template_model = notes::Index {
        notes: &note
    };

    let body = notes_template_model.render()?;
    let response = tide_response(200, api_response)
        .body(body)
        .content_type(mime::HTML)
        .build();

    Ok(response)
}

pub async fn new(_req: Request<AppState>) -> tide::Result {
    let template = New {};
    let body = template.render()?;
    let response = Response::builder(200)
        .body(body)
        .content_type(mime::HTML)
        .build();

    Ok(response)
}

pub async fn create(mut req: Request<AppState>) -> tide::Result {
    let new_note: NoteForm = req.body_form().await?;
    let api_request = surf_post_request(
        api_url_for_path(req.state(), "/notes"),
        &req,
    ).body_json(&new_note)?;
    let api_response: SurfResponse = api_request.await?;

    let response = tide_response(302, api_response)
        .header("Location", "/notes")
        .build();

    Ok(response)
}


pub async fn edit(req: Request<AppState>) -> tide::Result {
    let id = req.param("id")?;
    let api_request = surf_get_request(
        api_url_for_path(req.state(), format!("/notes/{}", id).as_str()),
        &req,
    );
    let mut api_response = api_request.await?;
    let note: Note = api_response.body_json().await?;

    let edit_template = Edit {
        note: &note
    };
    let body = edit_template.render()?;
    let response = tide_response(200, api_response)
        .body(body)
        .content_type(mime::HTML)
        .build();

    Ok(response)
}

pub async fn update(mut req: Request<AppState>) -> tide::Result {
    let note: NoteForm = req.body_form().await?;
    let id = req.param("id")?;

    let api_request = surf_put_request(
        api_url_for_path(req.state(), format!("/notes/{}", id).as_str()),
        &req,
    ).body_json(&note)?;
    let api_response = api_request.await?;

    let response = tide_response(302, api_response)
        .header("Location", format!("/notes/{}", id))
        .build();

    Ok(response)
}

pub async fn show(req: Request<AppState>) -> tide::Result {
    let id = req.param("id")?;
    let api_request = surf_get_request(
        api_url_for_path(req.state(), format!("/notes/{}", id).as_str()),
        &req,
    );
    let mut api_response = api_request.await?;
    let note: Note = api_response.body_json().await?;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(note.markdown.as_str(), options);

    let mut html_output: String = String::with_capacity(note.markdown.len() * 3 / 2);
    html::push_html(&mut html_output, parser);
    let show_template = Show {
        note: &note,
        markdown_html: &html_output,
    };
    let body = show_template.render()?;

    let response = tide_response(200, api_response)
        .body(body)
        .content_type(mime::HTML)
        .build();

    Ok(response)
}
