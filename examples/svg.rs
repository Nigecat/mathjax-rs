//! Renders the expression `f(x)=\frac{1}{x}` into the file `test.svg` with blue text.

fn main() {
    let renderer = mathjax::MathJax::new().unwrap();
    let mut res = renderer.render("f(x)=\\frac{1}{x}").unwrap();
    res.set_color("blue");
    std::fs::write("test.svg", res.into_raw()).unwrap();
}
