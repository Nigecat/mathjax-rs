#[derive(Debug, thiserror::Error)]
pub enum InitError {
    #[cfg(feature = "node")]
    #[error("unable to detect NodeJs v6 or later")]
    NodeJsUnavailable,
    #[cfg(feature = "node")]
    #[error("{0}")]
    ZipError(#[from] zip_extract::ZipExtractError),
    #[cfg(feature = "node")]
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[cfg(feature = "browser")]
    #[error("{0}")]
    Browser(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[cfg(feature = "node")]
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[cfg(feature = "node")]
    #[error("error in MathJax renderer: {0}")]
    MathJaxError(String),
    #[cfg(feature = "browser")]
    #[error("{0}")]
    Browser(#[from] anyhow::Error),
}
