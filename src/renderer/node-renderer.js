// Script designed to run as `node -e "SCRIPT_SOURCE" <expression>`,
//  prints the MathJax render of the given expression to stdout

// var document = undefined;

const { mathjax } = require("./mathjax");
const { TeX } = require("./input/tex");
const { SVG } = require("./output/svg");
const { liteAdaptor } = require("./adaptors/liteAdaptor");
const { RegisterHTMLHandler } = require("./handlers/html");
const { AllPackages } = require("./input/tex/AllPackages");

const expression = process.argv[1];
const adaptor = liteAdaptor();
RegisterHTMLHandler(adaptor);
const tex = new TeX({ packages: AllPackages });
const svg = new SVG({ fontCache: "local" });
const doc = mathjax.document("", { InputJax: tex, OutputJax: svg });
const node = doc.convert(expression);
const element = adaptor.innerHTML(node);
console.log(element);
