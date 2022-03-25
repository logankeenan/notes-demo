use tide::{Request, Response};
use crate::template_models::notes;
use askama::Template;
use pulldown_cmark::{Parser, Options, html};
use surf::RequestBuilder;
use tide::http::mime;
use crate::models::note::{NoteForm, Note};
use crate::AppState;
use crate::template_models::notes::{Edit, New, Show};

fn api_url_for_path(state: &AppState, path: &str) -> String {
    let api_url = state.environment.get("API_ORIGIN").unwrap();

    format!("{}{}", api_url, path)
}

fn add_cookie_to_request(tide_request: &Request<AppState>, api_request: RequestBuilder) -> RequestBuilder {
    match tide_request.header("cookie") {
        None => { api_request }
        Some(cookie_value) => {
            api_request.header("cookie", cookie_value)
        }
    }
}

pub async fn index(tide_request: Request<AppState>) -> tide::Result {
    let api_request: RequestBuilder = surf::get(api_url_for_path(tide_request.state(), "/notes"));
    let api_request: RequestBuilder = add_cookie_to_request(&tide_request, api_request);
    let mut api_response = api_request.await?;
    let note: Vec<Note> = api_response.body_json().await?;

    let notes_template_model = notes::Index {
        notes: &note
    };

    let body = notes_template_model.render()?;
    let mut response = Response::builder(200)
        .body(body)
        .content_type(mime::HTML)
        .build();

    match api_response.header("set-cookie") {
        None => {}
        Some(cookie_value) => {
            response.append_header("set-cookie", cookie_value);
        }
    }

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
    let api_request = surf::post(api_url_for_path(req.state(), "/notes")).body_json(&new_note)?;
    let api_request = add_cookie_to_request(&req, api_request);
    let api_response = api_request.await?;


    let mut response = Response::builder(302)
        .header("Location", "/notes")
        .build();
    match api_response.header("set-cookie") {
        None => {}
        Some(cookie_value) => {
            response.append_header("set-cookie", cookie_value);
        }
    }

    Ok(response)
}


pub async fn edit(req: Request<AppState>) -> tide::Result {
    let id = req.param("id")?;
    let api_request = surf::get(api_url_for_path(req.state(), format!("/notes/{}", id).as_str()));
    let api_request = add_cookie_to_request(&req, api_request);
    let mut api_response = api_request.await?;
    let note: Note = api_response.body_json().await?;

    let edit_template = Edit {
        note: &note
    };
    let body = edit_template.render()?;

    let mut response = Response::builder(200)
        .body(body)
        .content_type(mime::HTML)
        .build();

    match api_response.header("set-cookie") {
        None => {}
        Some(cookie_value) => {
            response.append_header("set-cookie", cookie_value);
        }
    }

    Ok(response)
}

pub async fn update(mut req: Request<AppState>) -> tide::Result {
    let note: NoteForm = req.body_form().await?;
    let id = req.param("id")?;

    let api_request = surf::put(api_url_for_path(req.state(), format!("/notes/{}", id).as_str()))
        .body_json(&note)?;
    let api_request = add_cookie_to_request(&req, api_request);
    let api_response = api_request.await?;

    let mut response = Response::builder(302).header("Location", format!("/notes/{}", id)).build();

    match api_response.header("set-cookie") {
        None => {}
        Some(cookie_value) => {
            response.append_header("set-cookie", cookie_value);
        }
    }
    Ok(response)
}

pub async fn show(req: Request<AppState>) -> tide::Result {
    let id = req.param("id")?;
    let api_request = surf::get(api_url_for_path(req.state(), format!("/notes/{}", id).as_str()));
    let api_request = add_cookie_to_request(&req, api_request);
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

    let mut response = Response::builder(200)
        .body(body)
        .content_type(mime::HTML)
        .build();

    match api_response.header("set-cookie") {
        None => {}
        Some(cookie_value) => {
            response.append_header("set-cookie", cookie_value);
        }
    }

    Ok(response)
}
