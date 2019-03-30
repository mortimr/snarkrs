use parser::ast::tokens;
use parser::ast;
use parser::program::symbol;
use parser::errors::logic::LogicError;

fn token_to_var_symbol(
    span: & (usize, usize),
    ctx: symbol::SymbolContext,
    collision: Vec<std::string::String>,
    name: std::string::String,
    expr: Option<& tokens::Token>)
    -> (symbol::SymbolContext, Vec<std::string::String>) {

    let mut collision = collision;
    let mut ctx = ctx;

    if collision.contains(&name) {
        ctx.errors.push(LogicError::build_with_span("Duplicate symbol detected: the following symbol already exists.".to_string(), 303, ctx.file.clone(), &ctx.source, (*span).clone()));
        return (ctx, collision);
    } else {
        let collision = & mut collision;
        collision.push(name.clone())
    }

    match expr {
        Some(expr) => {
            let mut ctx = ctx;

            ctx.node.borrow_mut().symbols.push(symbol::Symbol::GlobalVariable(symbol::GlobalVarSymbol {
                name,
                expr_ast: Some(expr.clone())
            }));

            (ctx, collision)

        },

        None => {

            let mut ctx = ctx;

            ctx.node.borrow_mut().symbols.push(symbol::Symbol::GlobalVariable(symbol::GlobalVarSymbol {
                name,
                expr_ast: None
            }));

            (ctx, collision)

        }
    }

}

fn load_global_var_from_tokens(
    span: & (usize, usize),
    rules: & Vec<tokens::Token>,
    ctx: symbol::SymbolContext,
    collision: Vec<std::string::String>)
    -> (symbol::SymbolContext, Vec<std::string::String>)
{
    match rules.as_slice() {

        // var name;

        [
        tokens::Token {
            rule: ast::Rule::VariableDeclarationKW,
            ..
        },
        tokens::Token {
            rule: ast::Rule::E_VariableName,
            content: tokens::TokenInfos::Terminal(name),
            ..
        }
        ] => {
            token_to_var_symbol(span, ctx, collision, name.content.clone(), None)
        },

        // var name = expr;

        [
        tokens::Token {
            rule: ast::Rule::VariableDeclarationKW,
            ..
        },
        tokens::Token {
            rule: ast::Rule::E_VariableName,
            content: tokens::TokenInfos::Terminal(name),
            ..
        },
        expr @ tokens::Token {
            rule: ast::Rule::Expression,
            ..
        }
        ] => {
            token_to_var_symbol(span, ctx, collision, name.content.clone(), Some(expr))
        },

        // signal name;  component name;

        _ => {
            let mut ctx = ctx;

            ctx.errors.push(LogicError::build_with_span("Invalid global declaration: only main component and vars allowed".to_string(), 302, ctx.file.clone(), &ctx.source, (*span).clone()));
            (ctx, collision)
        }

    }
}

fn load_function_from_tokens(
    span: & (usize, usize),
    rules: & Vec<tokens::Token>,
    ctx: symbol::SymbolContext,
    collision: Vec<std::string::String>)
    -> (symbol::SymbolContext, Vec<std::string::String>)
{
    match rules.as_slice() {
        [
        tokens::Token {
            rule: ast::Rule::FunctionKW,
            ..
        },
        tokens::Token {
            rule: ast::Rule::FunctionName,
            content: tokens::TokenInfos::Terminal(name),
            ..
        },
        params @ tokens::Token {
            rule: ast::Rule::Parameters,
            ..
        },
        body @ tokens::Token {
            rule: ast::Rule::Body,
            ..
        }
        ] => {
            let mut ctx = ctx;
            let mut collision = collision;

            if collision.contains(&name.content) {
                ctx.errors.push(LogicError::build_with_span("Duplicate symbol detected: the following symbol already exists.".to_string(), 303, ctx.file.clone(), &ctx.source, (*span).clone()));
                return (ctx, collision);
            } else {
                let collision = & mut collision;
                collision.push(name.content.clone())
            }

            ctx.node.borrow_mut().symbols.push(symbol::Symbol::Function(
                symbol::FunctionSymbol {
                    name: name.content.clone(),
                    function_body_ast: body.clone(),
                    function_params_ast: params.clone()
                }
            ));

            (ctx, collision)
        },
        _ => {
            panic!("An error occured in the parser. Malfored data was received in the AST.");
        }
    }
}

fn load_template_from_tokens(
    span: & (usize, usize),
    rules: & Vec<tokens::Token>,
    ctx: symbol::SymbolContext,
    collision: Vec<std::string::String>)
    -> (symbol::SymbolContext, Vec<std::string::String>)
{
    match rules.as_slice() {
        [
        tokens::Token {
            rule: ast::Rule::TemplateKW,
            ..
        },
        tokens::Token {
            rule: ast::Rule::TemplateName,
            content: tokens::TokenInfos::Terminal(name),
            ..
        },
        params @ tokens::Token {
            rule: ast::Rule::Parameters,
            ..
        },
        body @ tokens::Token {
            rule: ast::Rule::Body,
            ..
        }
        ] => {
            let mut ctx = ctx;
            let mut collision = collision;

            if collision.contains(&name.content) {
                ctx.errors.push(LogicError::build_with_span("Duplicate symbol detected: the following symbol already exists.".to_string(), 303, ctx.file.clone(), &ctx.source, (*span).clone()));
                return (ctx, collision);
            } else {
                let collision = & mut collision;
                collision.push(name.content.clone())
            }

            ctx.node.borrow_mut().symbols.push(symbol::Symbol::Template(
                symbol::TemplateSymbol {
                    name: name.content.clone(),
                    template_body_ast: body.clone(),
                    template_params_ast: params.clone()
                }
            ));

            (ctx, collision)
        },
        _ => {
            panic!("An error occured in the parser. Malfored data was received in the AST.");
        }
    }
}

fn load_symbols_from_circuit(
    circuit: & ast::tokens::NonTerminalToken,
    ctx: symbol::SymbolContext,
    collision: Vec<std::string::String>)
    -> (symbol::SymbolContext, Vec<std::string::String>)
{
    let (ctx, collision) = {

        let mut mut_ctx = ctx;
        let mut mut_collision = collision;

        for token in &circuit.subrules {

            let (ret_ctx, ret_collision) = match token {
                tokens::Token {
                    rule: ast::Rule::TemplateBlock,
                    content: tokens::TokenInfos::NonTerminal(template)
                } => {
                    load_template_from_tokens(&template.span, &template.subrules, mut_ctx, mut_collision)
                },
                tokens::Token {
                    rule: ast::Rule::FunctionBlock,
                    content: tokens::TokenInfos::NonTerminal(function)
                } => {
                    load_function_from_tokens(&function.span, &function.subrules, mut_ctx, mut_collision)
                },
                tokens::Token {
                    rule: ast::Rule::DeclarationStatement,
                    content: tokens::TokenInfos::NonTerminal(gvar),
                    ..
                } => {
                    load_global_var_from_tokens(&gvar.span, &gvar.subrules, mut_ctx, mut_collision)
                },
                tokens::Token {
                    rule: ast::Rule::IncludeStatement,
                    ..
                }
                |
                tokens::Token {
                    rule: ast::Rule::END_OF_LINE,
                    ..
                } => {
                    (mut_ctx, mut_collision)
                },
                _ => {
                    panic!("An error occured in the parser. Malfored data was received in the AST.");
                }
            };

            mut_ctx = ret_ctx;
            mut_collision = ret_collision;
        }

        (mut_ctx, mut_collision)
    };


    (ctx, collision)
}

pub fn load_symbols(
    ctx: symbol::SymbolContext,
    file: ast::File,
    collision: Vec<std::string::String>)
    -> (symbol::SymbolContext, Vec<std::string::String>)
{

    let (ctx, collision) = match file.root.ast.as_slice() {
        [
        ast::tokens::Token {
            rule: ast::Rule::Circuit,
            content: tokens::TokenInfos::NonTerminal(circuit),
            ..
        }
        ] => {
            load_symbols_from_circuit(circuit, ctx, collision)
        },
        _ => {
            panic!("An error occured in the parser. Malfored data was received in the AST.");
        }
    };

    (ctx, collision)

}
