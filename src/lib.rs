//! Render [MathJax](https://www.mathjax.org/) expressions into either SVG or PNG formats.
//!
//! # Crate Feature Flags
//!
//!  - `node` - Enables the [NodeJs](https://nodejs.org/) backend, this will attempt to use a system installation of NodeJs at runtime as the renderer. This will be priotized over any other features if enabled.
//!  - `browser` - Enables the [`headless_chrome`] backend, this will create a headless Chrome instance to use as the renderer. If this is enabled in conjunction with the `node` feature flag, this will be used as a fall back when NodeJs is not available.
//!  - `auto` - This is equivelent to enabling all backends (currently just `node` and `browser`) see [`MathJax::new`] for what this specifically does.
//!  - `image` - Allows converting the rendered SVG into an [`image::DynamicImage`] via [`Render::into_image`].
//!
//! By default, the `auto` crate feature is enabled.
//!
//! # Usage
//!
//! 1. Create an instance of [`MathJax`] through [`MathJax::new`].
//! 2. Call [`MathJax::render`] with the expression you want to render.
//! 3. Call one of the conversion methods on [`Render`] to get the desired output format.
//!
//! For example, if we wanted to render the expression `y=\frac{1}{x}` into the file `test.svg`:
//! ```rust
//! # fn main() {
//! use mathjax::MathJax;
//!
//! let expression = r#"y=\frac{1}{x}"#;
//! let renderer = MathJax::new().unwrap();
//! let result = renderer.render(expression).unwrap();
//! let svg_string = result.into_raw(); // This is a `<svg></svg>` element.
//! std::fs::write("test.svg", svg_string).unwrap();
//! # }
//! ```
//!
//! If we had the `image` feature flag enabled, we could do the following to convert into an [`image::DynamicImage`] and save it into the file `test.png`:
//! ```rust
//! # fn main() {
//! use mathjax::MathJax;
//!
//! let expression = r#"y=\frac{1}{x}"#;
//! let renderer = MathJax::new().unwrap();
//! let result = renderer.render(expression).unwrap();
//! let image = result.into_image(10.0).unwrap(); // This is an image::DynamicImage.
//! image.save("test.png").unwrap();
//! # }
//! ```
//! (see [`Render::into_image`] for what `10.0` means here)

#![cfg_attr(docs, feature(doc_auto_cfg))]
#![doc(html_root_url = "https://docs.rs/mathjax/0.1.0")]
#![warn(missing_docs)]
#[cfg(all(not(feature = "node"), not(feature = "browser")))]
compile_error!("No renderer enabled, at least one of either the `node` or `browser` feature flags must be enabled.");

mod error;
mod renderer;

pub use error::{InitError, RenderError};
pub use renderer::Render;
use renderer::Renderer;

/// The renderer.
pub struct MathJax {
    renderer: Renderer,
}

impl MathJax {
    /// Create a new renderer instance.
    ///
    /// Assuming you have the `auto` feature flag enabled, this function will perform the following:  
    /// 1. Check if NodeJs is installed and has a version greater than v6  
    ///
    /// **If the above is true:**  
    /// 2. Extract a copy of the NodeJs MathJax renderer to a temporary location on the system and return.  
    ///
    /// **Otherwise:**   
    /// 2. Create a [`headless_chrome::Browser`] instance and return.   
    ///    This instance will persist until this object is dropped and will be reused for repeated renders.  
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
            return Ok(MathJax {
                renderer: Renderer::Browser(renderer::browser::Browser::create()?),
            });
        }
    }

    /// Render the given [MathJax](https://www.mathjax.org/) expression into an image.
    pub fn render<S>(&self, expression: S) -> Result<Render, RenderError>
    where
        S: AsRef<str>,
    {
        let expression = expression.as_ref();

        match self.renderer {
            #[cfg(feature = "node")]
            Renderer::Node(ref node) => node.render(expression),
            #[cfg(feature = "browser")]
            Renderer::Browser(ref browser) => browser.render(expression),
        }
    }
}
