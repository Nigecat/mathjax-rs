#[cfg(feature = "browser")]
pub mod browser;
#[cfg(feature = "node")]
pub mod node;

/// The output of a renderer, this is the final [MathJax](https://www.mathjax.org/) image.
pub struct Render {
    /// The actual SVG source that MathJax outputs
    source: String,
}

impl Render {
    fn new(source: String) -> Self {
        Render { source }
    }

    /// Returns the underlying SVG string. This is an `<svg>...</svg>` element.
    pub fn into_raw(self) -> String {
        self.source
    }

    /// Converts the render into a [`resvg::Tree`].
    #[cfg(feature = "image")]
    pub fn into_svg(self) -> resvg::Tree {
        unimplemented!(); // todo
    }

    /// Converts the render into an [`image::DynamicImage`].
    #[cfg(feature = "image")]
    pub fn into_image(self) -> image::DynamicImage {
        unimplemented!(); // todo
    }
}
