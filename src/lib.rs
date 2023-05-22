#![cfg_attr(docs, feature(doc_auto_cfg))]
#[cfg(all(not(feature = "node"), not(feature = "browser")))]
compile_error!("No renderer enabled, at least one of either the `node` or `browser` feature flags must be enabled.");

mod error;
mod renderer;

pub use error::{InitError, RenderError};
pub use renderer::Render;
use renderer::Renderer;

pub struct MathJax {
    renderer: Renderer,
}

impl MathJax {
    #[allow(clippy::needless_return)]
    pub fn new() -> Result<Self, InitError> {
        #[cfg(feature = "node")]
        if renderer::node::available() {
            return Ok(MathJax {
                renderer: Renderer::Node(renderer::node::Node::create()?),
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

        match self.renderer {
            Renderer::Node(ref node) => node.render(expression),
        }
    }
}
