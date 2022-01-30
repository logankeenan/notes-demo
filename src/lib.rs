use std::collections::HashMap;
use tide::Server;

mod router;
mod controllers;
mod template_models;
mod models;

#[derive(Clone)]
pub struct AppState {
    pub environment: HashMap<String, String>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            environment: Default::default()
        }
    }
}

pub fn create(state: AppState) -> Server<AppState> {
    let mut app = tide::with_state(state);
    router::setup(&mut app);

    app
}
