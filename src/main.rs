fn main() {
    let renderer = mathjax::MathJax::new().unwrap();
    let res = renderer.render("f(x)=\\frac{1}{x}").unwrap();
    println!("{}", res.into_raw());
}
