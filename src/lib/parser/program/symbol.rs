use parser::ast::tokens;
use parser::errors;
use std::rc;
use std::collections;
use std::cell;

pub type ModuleStore = rc::Rc<cell::RefCell<collections::HashMap<std::path::PathBuf, rc::Rc<cell::RefCell<SymbolNode>>>>>;

#[derive(Debug)]
pub struct SymbolContext {
    pub node: rc::Rc<cell::RefCell<SymbolNode>>,
    pub errors: Vec<errors::CompileError>,
    pub module_store: ModuleStore,
    pub source: std::string::String,
    pub file: std::path::PathBuf
}

#[derive(Debug)]
pub struct MainSymbol {
    pub name: std::string::String,
    pub expr_ast: tokens::Token
}

#[derive(Debug)]
pub struct TemplateSymbol {
    pub name: std::string::String,
    pub template_params_ast: tokens::Token,
    pub template_body_ast: tokens::Token
}

#[derive(Debug)]
pub struct GlobalVarSymbol {
    pub name: std::string::String,
    pub expr_ast: Option<tokens::Token>
}

#[derive(Debug)]
pub struct FunctionSymbol {
    pub name: std::string::String,
    pub function_params_ast: tokens::Token,
    pub function_body_ast: tokens::Token
}

#[derive(Debug)]
pub enum Symbol {
    Main(MainSymbol),
    Template(TemplateSymbol),
    Function(FunctionSymbol),
    GlobalVariable(GlobalVarSymbol)
}

#[derive(Debug)]
pub struct SymbolNode {
    pub symbols: Vec<Symbol>,
    pub modules: Vec<rc::Rc<SymbolNode>>
}

