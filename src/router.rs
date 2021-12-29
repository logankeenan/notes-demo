use tide::Server;
use crate::controllers::note_controller;

pub fn setup(app: &mut Server<()>) {
    app.at("/notes").get(note_controller::index);
}