use parser::ast::tokens::*;
use parser::ast::Rule;

pub fn process_include_path_string(include_rules: & Vec<Token>) -> & str {

    match include_rules.as_slice() {
        [
        Token {
            rule: Rule::FilesystemPath,
            content: TokenInfos::Terminal(ctn),
            ..
        }
        ] => &ctn.content,
        _ => panic!("Invalid IncludePathString AST Token")
    }

}

pub fn process_include_statement(include_statement: & Token) -> & str{

    match include_statement {
        Token {
            content: TokenInfos::NonTerminal(is_infos),
            ..
        } => {

            match is_infos.subrules.as_slice() {
                [
                Token {rule: Rule::IncludeKW, ..},
                Token{
                    rule: Rule::IncludePathString,
                    content: TokenInfos::NonTerminal(ips),
                    ..
                },
                Token{ rule: Rule::END_OF_LINE, .. }
                ] => process_include_path_string(&ips.subrules),
                _ => panic!("Invalid IncludeStatement AST Token")
            }

        },
        _ => {
            panic!("Expected NonTerminal Token for IncludeStatement")
            //Error lol
        }
    }


}

