//! Renders the expression `f(x)=\frac{1}{x}` into the file `test.png` with green text.

fn main() {
    let renderer = mathjax::MathJax::new().unwrap();
    let mut res = renderer.render("f(x)=\\frac{1}{x}").unwrap();
    // res.set_color("green");
    res.into_image(10.0).unwrap().save("test.png").unwrap();
}
