mod parse;
mod prepare;
mod run;
mod types;

use rustpython_parser::ast::Constant;

use crate::parse::parse;
use crate::prepare::prepare;
use crate::run::run;

fn main() {
    let code = "a = 1\nb = 1\nif a == b:\n    print('yes')\n";
    let nodes = parse(code, None).unwrap();
    dbg!(&nodes);
    let (namespace_size, nodes) = prepare(nodes).unwrap();
    // dbg!(namespace_size, &nodes);
    run(namespace_size, &nodes).unwrap();
}
