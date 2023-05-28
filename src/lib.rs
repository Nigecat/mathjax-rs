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
//! Which produces the following image:
//! <svg style="vertical-align: -1.577ex;" xmlns="http://www.w3.org/2000/svg" width="9.605ex" height="4.613ex" role="img" focusable="false" viewBox="0 -1342 4245.6 2039" xmlns:xlink="http://www.w3.org/1999/xlink"><defs><path id="MJX-1-TEX-I-1D453" d="M118 -162Q120 -162 124 -164T135 -167T147 -168Q160 -168 171 -155T187 -126Q197 -99 221 27T267 267T289 382V385H242Q195 385 192 387Q188 390 188 397L195 425Q197 430 203 430T250 431Q298 431 298 432Q298 434 307 482T319 540Q356 705 465 705Q502 703 526 683T550 630Q550 594 529 578T487 561Q443 561 443 603Q443 622 454 636T478 657L487 662Q471 668 457 668Q445 668 434 658T419 630Q412 601 403 552T387 469T380 433Q380 431 435 431Q480 431 487 430T498 424Q499 420 496 407T491 391Q489 386 482 386T428 385H372L349 263Q301 15 282 -47Q255 -132 212 -173Q175 -205 139 -205Q107 -205 81 -186T55 -132Q55 -95 76 -78T118 -61Q162 -61 162 -103Q162 -122 151 -136T127 -157L118 -162Z"></path><path id="MJX-1-TEX-N-28" d="M94 250Q94 319 104 381T127 488T164 576T202 643T244 695T277 729T302 750H315H319Q333 750 333 741Q333 738 316 720T275 667T226 581T184 443T167 250T184 58T225 -81T274 -167T316 -220T333 -241Q333 -250 318 -250H315H302L274 -226Q180 -141 137 -14T94 250Z"></path><path id="MJX-1-TEX-I-1D465" d="M52 289Q59 331 106 386T222 442Q257 442 286 424T329 379Q371 442 430 442Q467 442 494 420T522 361Q522 332 508 314T481 292T458 288Q439 288 427 299T415 328Q415 374 465 391Q454 404 425 404Q412 404 406 402Q368 386 350 336Q290 115 290 78Q290 50 306 38T341 26Q378 26 414 59T463 140Q466 150 469 151T485 153H489Q504 153 504 145Q504 144 502 134Q486 77 440 33T333 -11Q263 -11 227 52Q186 -10 133 -10H127Q78 -10 57 16T35 71Q35 103 54 123T99 143Q142 143 142 101Q142 81 130 66T107 46T94 41L91 40Q91 39 97 36T113 29T132 26Q168 26 194 71Q203 87 217 139T245 247T261 313Q266 340 266 352Q266 380 251 392T217 404Q177 404 142 372T93 290Q91 281 88 280T72 278H58Q52 284 52 289Z"></path><path id="MJX-1-TEX-N-29" d="M60 749L64 750Q69 750 74 750H86L114 726Q208 641 251 514T294 250Q294 182 284 119T261 12T224 -76T186 -143T145 -194T113 -227T90 -246Q87 -249 86 -250H74Q66 -250 63 -250T58 -247T55 -238Q56 -237 66 -225Q221 -64 221 250T66 725Q56 737 55 738Q55 746 60 749Z"></path><path id="MJX-1-TEX-N-3D" d="M56 347Q56 360 70 367H707Q722 359 722 347Q722 336 708 328L390 327H72Q56 332 56 347ZM56 153Q56 168 72 173H708Q722 163 722 153Q722 140 707 133H70Q56 140 56 153Z"></path><path id="MJX-1-TEX-N-31" d="M213 578L200 573Q186 568 160 563T102 556H83V602H102Q149 604 189 617T245 641T273 663Q275 666 285 666Q294 666 302 660V361L303 61Q310 54 315 52T339 48T401 46H427V0H416Q395 3 257 3Q121 3 100 0H88V46H114Q136 46 152 46T177 47T193 50T201 52T207 57T213 61V578Z"></path></defs><g stroke="currentColor" fill="currentColor" stroke-width="0" transform="scale(1,-1)"><g data-mml-node="math"><g data-mml-node="mi"><use data-c="1D453" xlink:href="#MJX-1-TEX-I-1D453"></use></g><g data-mml-node="mo" transform="translate(550,0)"><use data-c="28" xlink:href="#MJX-1-TEX-N-28"></use></g><g data-mml-node="mi" transform="translate(939,0)"><use data-c="1D465" xlink:href="#MJX-1-TEX-I-1D465"></use></g><g data-mml-node="mo" transform="translate(1511,0)"><use data-c="29" xlink:href="#MJX-1-TEX-N-29"></use></g><g data-mml-node="mo" transform="translate(2177.8,0)"><use data-c="3D" xlink:href="#MJX-1-TEX-N-3D"></use></g><g data-mml-node="mfrac" transform="translate(3233.6,0)"><g data-mml-node="mn" transform="translate(256,676)"><use data-c="31" xlink:href="#MJX-1-TEX-N-31"></use></g><g data-mml-node="mi" transform="translate(220,-686)"><use data-c="1D465" xlink:href="#MJX-1-TEX-I-1D465"></use></g><rect width="772" height="60" x="120" y="220"></rect></g></g></g></svg>
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
