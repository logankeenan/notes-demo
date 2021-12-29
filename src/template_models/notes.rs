use askama::Template; // bring trait in scope

#[derive(Template)] // this will generate the code...
#[template(path = "notes/index.html")] // using the te
pub struct Index {}