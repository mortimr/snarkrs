use std::rc;
use parser::ast;
use parser::program::symbol;
use std::cell;
use std::collections::HashMap;
use parser::program::symbol::{SymbolNode, SymbolContext};
use parser::program::main_symbol_loader;
use parser::program::include_symbol_loader;
use parser::program::symbol_loader;

use parser::errors::parse::ParseError;
use parser::errors::io::IOError;


fn build_main(main: std::path::PathBuf) -> SymbolContext {

    // Create current Symbol Node
    let node: rc::Rc<cell::RefCell<symbol::SymbolNode>> = rc::Rc::new(
        cell::RefCell::new(symbol::SymbolNode {
            symbols: Vec::new(),
            modules: Vec::new()
        })
    );

    // Create shared mutable Symbol Node store
    let module_store: rc::Rc<cell::RefCell<HashMap<std::path::PathBuf, rc::Rc<cell::RefCell<SymbolNode>>>>> = rc::Rc::new(
        cell::RefCell::new(
            HashMap::new()
        )
    );

    // Create current Symbol Context
    let mut ctx: symbol::SymbolContext = symbol::SymbolContext {
        node,
        errors: Vec::new(),
        module_store,
        source: std::string::String::new(),
        file: main.clone()
    };

    let main = match std::fs::canonicalize(main.clone()) {
        Ok(new_path) => new_path,
        Err(err) => {
            ctx.errors.push(IOError::build("IO Error occured !".to_string(), 201, err, &main));
            return ctx;
        }
    };

    ctx.file = main.clone();

    let (file, source): (ast::File, std::string::String) = match ast::parse_file(& main.clone()) {
        Ok(res) => res,
        Err(error) => {
            match error {
                ast::ASTError::PestError(pest, content) => {
                    ctx.errors.push(ParseError::build("Syntax error detected near".to_string(), 101, pest, &content));
                },
                ast::ASTError::IOError(error) => {
                    ctx.errors.push(IOError::build("IO Error occured !".to_string(), 201, error, &main.clone()));
                }
            }
            return ctx;
        }
    };

    ctx.source = source;
    (*ctx.module_store).borrow_mut().insert(main.clone(), ctx.node.clone());

    // Load main symbol from input file and remove it from ast
    let (ctx, file) = main_symbol_loader::load_main_symbol(ctx, file);

    if ctx.errors.len() != 0 {
        return ctx;
    }

    let mut collision: Vec<std::string::String> = Vec::new();
    collision.push(std::string::String::from("main"));

    // Load include statements
    let (ctx, file, collision) = include_symbol_loader::load_includes(ctx, file, collision);

    if ctx.errors.len() != 0 {
        return ctx;
    }

    // Load symbols
    let (ctx, _collision) = symbol_loader::load_symbols(ctx, file, collision);

    ctx
}

pub fn build(main: std::path::PathBuf) -> SymbolContext {
    build_main(main)
}

#[cfg(test)]
mod build_test {

    use galvanic_assert::matchers::*;
    use parser::program::build;

    #[test]
    fn succesful_program_build() {
        let ctx = build::build(std::path::PathBuf::from("./src/lib/parser/test_material/circuits/main_bitify.circom"));

        expect_that!(&ctx.errors.len(), eq(0));

    }

}