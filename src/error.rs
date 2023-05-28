/// An error during the initialization of the renderer backend.
#[derive(Debug, thiserror::Error)]
pub enum InitError {
    /// [NodeJs](https://nodejs.org/) v6 or later was not found on the system.
    #[cfg(all(feature = "node", not(feature = "browser")))]
    #[error("unable to detect NodeJs v6 or later")]
    NodeJsUnavailable,
    /// Unable to extract MathJax source archive, this is probably a permission error.
    #[cfg(feature = "node")]
    #[error("{0}")]
    ZipError(#[from] zip_extract::ZipExtractError),
    #[cfg(feature = "node")]
    /// An IO error.
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    /// An error originating from the underlying [`headless_chrome`] instance.
    #[cfg(feature = "browser")]
    #[error("{0}")]
    Browser(#[from] anyhow::Error),
}

/// An error during a render.
#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    /// An IO error.
    #[cfg(feature = "node")]
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    /// An error from the NodeJs renderer.
    #[cfg(feature = "node")]
    #[error("error in MathJax renderer: {0}")]
    MathJaxError(String),
    /// An error originating from the underlying [`headless_chrome`] instance.
    #[cfg(feature = "browser")]
    #[error("{0}")]
    Browser(#[from] anyhow::Error),
}
