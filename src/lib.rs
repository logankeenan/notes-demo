use tide::Server;

mod router;
mod controllers;
mod template_models;

pub fn create() -> Server<()> {
    let mut app = tide::new();
    router::setup(&mut app);

    app
}
