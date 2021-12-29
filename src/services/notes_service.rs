use surf::Error;
use crate::models::note::Note;

pub async fn all_notes() -> Vec<Note>{
    let result: Result<Vec<Note>, Error> = surf::get("http://localhost:3000/notes").recv_json().await;

    match result {
        Ok(response) => {
           response
        }
        Err(_) => {
            // TODO handle some error
            Vec::new()
        }
    }
}