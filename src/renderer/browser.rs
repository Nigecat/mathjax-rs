const CDN: &str = "https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-svg-full.js";

pub struct Browser {
    instance: headless_chrome::Browser,
}

impl Browser {
    pub fn create() -> Result<Self, crate::InitError> {
        let instance = headless_chrome::Browser::default()?;
        Ok(Browser { instance })
    }

    pub fn render(&self, expression: &str) -> Result<super::Render, crate::RenderError> {
        let browser = &self.instance;

        let tab = browser.new_tab()?;
        let body = tab
            .navigate_to("about:blank")?
            .wait_until_navigated()?
            .find_element("body")?;

        // Load MathJax and render
        let svg = body
            .call_js_fn(
                &r#"(expression) => {
            return new Promise((resolve, reject) => {
                window.MathJax = {
                    startup: {
                        ready: () => {
                            MathJax.startup.defaultReady();
                            const svg = window.MathJax.tex2svg(expression).children[0].outerHTML;
                            resolve(svg);
                        }
                    }
                };

                let mj = document.createElement("script");
                mj.setAttribute("src", "<CDN>");
                mj.setAttribute("type", "text/javascript");
                mj.setAttribute("id", "MathJax-script");
                mj.setAttribute("crossorigin", "anonymous");
                mj.addEventListener("error", (err) => reject(err));
                document.head.appendChild(mj);
            });
        }"#
                .replace("<CDN>", CDN),
                vec![expression.into()],
                true,
            )?
            .value
            .and_then(|value| value.as_str().map(ToString::to_string))
            .unwrap_or_default();

        Ok(super::Render::new(svg))
    }
}
