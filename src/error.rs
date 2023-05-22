#[derive(Debug, thiserror::Error)]
pub enum InitError {
    #[error("unable to detect NodeJs v6 or later")]
    NodeJsUnavailable,
}

#[derive(Debug, thiserror::Error)]
pub enum RenderError {}
