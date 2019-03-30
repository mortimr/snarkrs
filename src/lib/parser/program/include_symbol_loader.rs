use parser::ast;
use parser::program::symbol;

pub fn load_includes(
    ctx: symbol::SymbolContext,
    file: ast::File,
    collision: Vec<std::string::String>)
    -> (symbol::SymbolContext, ast::File, Vec<std::string::String>)
{

    (ctx, file, collision)

}
