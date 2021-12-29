use tide::Server;
use crate::controllers::note_controller;

pub fn setup(app: &mut Server<()>) {
    app.at("/notes").get(note_controller::index);
    app.at("/notes/new").get(note_controller::new);
    app.at("/notes").post(note_controller::create);
}