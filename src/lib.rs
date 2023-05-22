#[cfg(all(not(feature = "node"), not(feature = "browser")))]
compile_error!("No renderer enabled, at least one of either the `node` or `browser` feature flags must be enabled.");

mod error;
mod renderer;

pub use error::{InitError, RenderError};
pub use renderer::Render;

pub struct MathJax {
    /// Whether NodeJs was available when this object was created
    #[cfg(feature = "node")]
    node_available: bool,
}

impl MathJax {
    pub fn new() -> Result<Self, InitError> {
        #[cfg(feature = "node")]
        if renderer::node::available() {
            return Ok(MathJax {
                node_available: true,
            });
        } else {
            #[cfg(not(feature = "browser"))]
            return Err(InitError::NodeJsUnavailable);
        }

        #[cfg(feature = "browser")]
        {
            // todo
        }
    }

    pub fn render<S>(&self, expression: S) -> Result<Render, RenderError>
    where
        S: AsRef<str>,
    {
        let expression = expression.as_ref();

        #[cfg(feature = "node")]
        if self.node_available {
            return renderer::node::render(expression);
        }

        #[cfg(feature = "browser")]
        {
            // todo
        }

        unimplemented!();
    }
}
