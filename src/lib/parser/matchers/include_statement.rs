use parser::ast::tokens::*;
use parser::ast::Rule;

pub fn process_include_path_string(include_path_string: & NonTerminalToken) -> & str {

    match include_path_string.subrules.as_slice() {
        [
        Token::Terminal(TerminalToken{rule: Rule::FilesystemPath, content, ..})
        ] => content,
        _ => panic!("Invalud IncludePathString AST Token")
    }

}

pub fn process_include_statement(include_statement: & NonTerminalToken) -> & str{

    match include_statement.subrules.as_slice() {
        [
        Token::Terminal(TerminalToken {rule: Rule::IncludeKW, ..}),
        Token::NonTerminal( ips @
            NonTerminalToken
            {
                rule: Rule::IncludePathString,
                ..
            }
        ),
        Token::Terminal(TerminalToken {rule: Rule::END_OF_LINE, ..})
        ] => process_include_path_string(ips),
        _ => panic!("Invalid IncludeStatement AST Token")
    }

}

