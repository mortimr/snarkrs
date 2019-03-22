extern crate libsnarkrs;

use libsnarkrs::parser;

fn main() {
    match parser::parse("include \"lele\";", None) {
        Ok(parsed) => parser::recurse_down(parsed, 0),
        _ => panic!("Error while parsing")
    }
}

