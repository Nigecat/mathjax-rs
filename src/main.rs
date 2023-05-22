fn main() {
    let renderer = mathjax::MathJax::new().unwrap();
    let mut res = renderer.render("f(x)=\\frac{1}{x}").unwrap();
    res.set_color("white");
    std::fs::write("test.svg", res.into_raw()).unwrap();
}
