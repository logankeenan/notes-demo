use askama::Template;
use crate::models::note::Note; // bring trait in scope

#[derive(Template)]
#[template(path = "notes/index.html")]
pub struct Index<'a> {
    pub notes: &'a Vec<Note>,
}

#[derive(Template)]
#[template(path = "notes/new.html")]
pub struct New {}


#[derive(Template)]
#[template(path = "notes/edit.html")]
pub struct Edit<'a> {
    pub note: &'a Note,
}
