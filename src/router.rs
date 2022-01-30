use tide::Server;
use crate::controllers::note_controller;
use crate::AppState;

pub fn setup(app: &mut Server<AppState>) {
    app.at("/notes").get(note_controller::index);
    app.at("/").get(note_controller::index);
    app.at("/notes/new").get(note_controller::new);
    app.at("/notes").post(note_controller::create);
    app.at("/notes/edit/:id").get(note_controller::edit);
    app.at("/notes/:id").post(note_controller::update);
    app.at("/notes/:id").get(note_controller::show);
}