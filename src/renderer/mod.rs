#[cfg(feature = "browser")]
pub mod browser;
#[cfg(feature = "node")]
pub mod node;

pub enum Renderer {
    #[cfg(feature = "node")]
    Node(node::Node),
    #[cfg(feature = "browser")]
    Browser(browser::Browser),
}

/// The output of a renderer, this is the final [MathJax](https://www.mathjax.org/) image.
pub struct Render {
    /// The actual SVG source that MathJax outputs
    source: String,
    /// Whether the text/line color has been set
    color_set: bool,
}

impl Render {
    fn new(source: String) -> Self {
        Render {
            source,
            color_set: false,
        }
    }

    /// Sets the text/line color of the rendered image.  
    ///
    /// Will return `true` if the operation was successful.
    /// This function can only be called once, subsequent calls will do nothing and return `false`.
    ///
    /// Accepts any valid CSS [color](https://developer.mozilla.org/en-US/docs/Web/CSS/color_value) value.
    pub fn set_color(&mut self, color: &str) -> bool {
        if !self.color_set {
            self.source = self.source.replace("currentColor", color);
            self.color_set = true;
            true
        } else {
            false
        }
    }

    /// Returns the underlying SVG string. This is an `<svg>...</svg>` element.
    pub fn raw(&self) -> &str {
        &self.source
    }

    /// Converts the render into the underlying SVG string. This is an `<svg>...</svg>` element.
    pub fn into_raw(self) -> String {
        self.source
    }

    /// Converts the render into a [`resvg::Tree`].
    #[cfg(feature = "image")]
    pub fn into_svg(self) -> Result<resvg::Tree, resvg::usvg::Error> {
        use resvg::usvg::{TreeParsing, TreeTextToPath};

        let opt = resvg::usvg::Options::default();

        let mut fontdb = resvg::usvg::fontdb::Database::new();
        fontdb.load_system_fonts();

        let mut tree = resvg::usvg::Tree::from_data(self.source.as_bytes(), &opt)?;
        tree.convert_text(&fontdb);
        let rtree = resvg::Tree::from_usvg(&tree);
        Ok(rtree)
    }

    /// Converts the render into an [`image::DynamicImage`].
    #[cfg(feature = "image")]
    pub fn into_image(self, scaling_factor: f32) -> Result<image::DynamicImage, image::ImageError> {
        // todo don't unwrap
        let rtree = self.into_svg().unwrap();

        let pixmap_size = resvg::IntSize::from_usvg(rtree.size)
            .scale_by(scaling_factor.into())
            .unwrap();

        let mut pixmap =
            resvg::tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
        rtree.render(
            resvg::tiny_skia::Transform::from_scale(scaling_factor, scaling_factor),
            &mut pixmap.as_mut(),
        );

        image::load_from_memory_with_format(&pixmap.encode_png().unwrap(), image::ImageFormat::Png)
    }
}
