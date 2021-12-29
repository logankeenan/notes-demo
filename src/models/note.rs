use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub markdown: String,
    pub update_at: Option<String>,
    pub created_at: String
}

#[derive(Deserialize, Serialize)]
pub struct NoteForm {
    pub title: String,
    pub markdown: String,
}
