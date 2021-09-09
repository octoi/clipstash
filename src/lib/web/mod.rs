pub mod ctx;
pub mod renderer;
pub mod form;
pub mod http;

use handlebars::RenderError;

#[derive(rocket::Responder)]
pub enum PageError {
    #[response(status = 500)]
    SerializationError(String),
    #[response(status = 500)]
    Render(String),
    #[response(status = 404)]
    NoteFound(String),
    #[response(status = 500)]
    Internal(String)
}

impl From<handlebars::RenderError> for PageError {
    fn from(err: RenderError) -> Self {
        PageError::Render(format!("{}", err))
    }
}

impl From<serde_json::Error> for PageError {
    fn from(err: serde_json::Error) -> Self {
        PageError::Render(format!("{}", err))
    }
}
