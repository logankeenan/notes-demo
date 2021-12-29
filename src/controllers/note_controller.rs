use tide::{Request, Response};
use crate::template_models::notes;
use askama::Template;
use pulldown_cmark::{Parser, Options, html};
use tide::http::mime;
use crate::models::note::{NoteForm, Note};
use crate::template_models::notes::{Edit, New, Show};

pub async fn index(_req: Request<()>) -> tide::Result {
    let notes: Vec<Note> = surf::get("http://localhost:3000/notes").recv_json().await?;
    let notes_template_model = notes::Index {
        notes: &notes
    };
    let body = notes_template_model.render()?;
    let response = Response::builder(200)
        .body(body)
        .content_type(mime::HTML)
        .build();

    Ok(response)
}

pub async fn new(_req: Request<()>) -> tide::Result {
    let template = New {};
    let body = template.render()?;
    let response = Response::builder(200)
        .body(body)
        .content_type(mime::HTML)
        .build();

    Ok(response)
}

pub async fn create(mut req: Request<()>) -> tide::Result {
    let new_note: NoteForm = req.body_form().await?;
    let api_result = surf::post("http://localhost:3000/notes").body_json(&new_note)?.await;

    match api_result {
        Ok(_) => {
            let response = Response::builder(302)
                .header("Location", "/notes")
                .build();

            Ok(response)
        }
        Err(_) => {
            // TODO better error handling
            Ok(Response::from("an error occurred"))
        }
    }
}


pub async fn edit(req: Request<()>) -> tide::Result {
    let id = req.param("id")?;
    let note: Note = surf::get(format!("http://localhost:3000/notes/{}", id)).recv_json().await?;

    let edit_template = Edit {
        note: &note
    };
    let body = edit_template.render()?;

    let response = Response::builder(200)
        .body(body)
        .content_type(mime::HTML)
        .build();
    Ok(response)
}

pub async fn update(mut req: Request<()>) -> tide::Result {
    let note: NoteForm = req.body_form().await?;
    let id = req.param("id")?;

    let api_result = surf::put(format!("http://localhost:3000/notes/{}", id))
        .body_json(&note)?
        .await;

    match api_result {
        Ok(_) => {
            Ok(Response::builder(302).header("Location", format!("/notes/{}", id)).build())
        }
        Err(_) => {
            Ok(Response::from("An error occurred"))
        }
    }
}

pub async fn show(req: Request<()>) -> tide::Result {
    let id = req.param("id")?;
    let note: Note = surf::get(format!("http://localhost:3000/notes/{}", id))
        .recv_json()
        .await?;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(note.markdown.as_str(), options);

    let mut html_output: String = String::with_capacity(note.markdown.len() * 3 / 2);
    html::push_html(&mut html_output, parser);
    let show_template = Show {
        note: &note,
        markdown_html: &html_output
    };
    let body = show_template.render()?;

    let response = Response::builder(200)
        .body(body)
        .content_type(mime::HTML)
        .build();
    Ok(response)
}
