#[cfg(feature = "browser")]
pub mod browser;
#[cfg(feature = "node")]
pub mod node;

pub enum Renderer {
    #[cfg(feature = "node")]
    Node(node::Node),
    // #[cfg(feature = "browser")]
    // Browser,
}

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
    pub fn into_image(self) -> Result<image::DynamicImage, image::ImageError> {
        // let rtree = self.into_svg().unwrap();

        // let pixmap_size = resvg::IntSize::from_usvg(rtree.size);
        // let mut pixmap =
        //     resvg::tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
        // rtree.render(resvg::tiny_skia::Transform::default(), &mut pixmap.as_mut());

        // Ok(image::load_from_memory_with_format(
        //     &pixmap.encode_png().unwrap(),
        //     image::ImageFormat::Png,
        // )
        // .unwrap())
        unimplemented!(); // todo - extra care needs to be taken with scaling
    }
}
