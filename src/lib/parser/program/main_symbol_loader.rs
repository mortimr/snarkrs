use parser::ast::tokens;
use parser::ast;
use parser::errors;
use parser::program::symbol;
use parser::errors::logic::LogicError;

fn load_main_symbol_from_declaration_tokens(
    rules: & Vec<tokens::Token>,
    errors: Vec<errors::CompileError>)
    -> (Option<symbol::Symbol>, Vec<errors::CompileError>, bool)
{
    match rules.as_slice() {
        [
        tokens::Token {
            rule: ast::Rule::ComponentDeclarationKW,
            ..
        },
        tokens::Token {
            rule: ast::Rule::E_VariableName,
            content: tokens::TokenInfos::Terminal(name),
            ..
        },
        exp @ tokens::Token {
            rule: ast::Rule::Expression,
            ..
        }] => {
            if name.content.eq("main") {
                (Some(symbol::Symbol::Main(symbol::MainSymbol {
                    name: name.content.clone(),
                    expr_ast: (*exp).clone()
                })), errors, true)
            } else {
                (None, errors, false)
            }
        },
        _ => {
            (None, errors, false)
        }
    }
}

fn load_main_symbol_from_circuit_tokens(
    main_file: std::path::PathBuf,
    rules: & mut Vec<tokens::Token>,
    errors: Vec<errors::CompileError>)
    -> (Option<symbol::Symbol>, Vec<errors::CompileError>)
{

    let mut mut_errors = errors;
    for idx in 0..rules.len() {

        if idx < rules.len() -1 && rules[idx].rule == ast::Rule::DeclarationStatement && rules[idx + 1].rule == ast::Rule::END_OF_LINE {
            let (symbol, errors, found) = match &rules[idx].content {
                tokens::TokenInfos::NonTerminal(main) => {
                    load_main_symbol_from_declaration_tokens(&main.subrules, mut_errors)
                }
                _ => {
                    (None, mut_errors, false)
                }
            };

            mut_errors = errors;

            if found == true {
                rules.remove(idx);
                rules.remove(idx);

                return (symbol, mut_errors);
            }
        }

    }

    mut_errors.push(LogicError::build("Could not find a main component definition".to_string(), 301, main_file));

    (None, mut_errors)
}

fn load_main_symbol_from_ast(
    file: ast::File,
    errors: Vec<errors::CompileError>)
    -> (ast::File, Option<symbol::Symbol>, Vec<errors::CompileError>)
{

    let mut file : ast::File = file;

    let (symbol, errors) = match file.root.ast.as_mut_slice() {
        [
        tokens::Token {
            rule: ast::Rule::Circuit,
            content: tokens::TokenInfos::NonTerminal(circuit_content),
            ..
        }
        ] => {
            load_main_symbol_from_circuit_tokens(file.path.clone(), & mut circuit_content.subrules, errors)
        },
        _ => {
            panic!("An error occured in the parser. Malfored data was received in the AST.");
        }
    };

    (file, symbol, errors)
}

pub fn load_main_symbol(ctx: symbol::SymbolContext, file: ast::File)
                        -> (symbol::SymbolContext, ast::File)
{

    let (file, main, errors) = load_main_symbol_from_ast(file, ctx.errors);

    let node = ctx.node;

    match main {
        Some(expr) => {
            node.borrow_mut().symbols.push(expr);
        },
        None => {}
    }

    let node = node;

    (symbol::SymbolContext {node, errors, module_store: ctx.module_store, file: ctx.file, source: ctx.source}, file)
}
