use pest::Parser;

#[derive(Parser)]
#[grammar = "lib/parser/grammar.pest"]
struct CircuitParser;

pub mod tokens {

    use parser::ast;

    ///
    /// Enumeration containing the two types of AST tokens.
    ///
    #[derive(Debug)]
    pub enum Token {
        Terminal(TerminalToken),
        NonTerminal(NonTerminalToken)
    }

    ///
    /// NonTerminal Tokens: tokens that contain sub tokens. In our case, the cpatured value is ignored.
    ///
    #[derive(Debug)]
    pub struct NonTerminalToken {
        pub span: (usize, usize),
        pub rule: ast::Rule,
        pub subrules: Vec<Token>
    }

    ///
    /// Terminal Tokens: tokens that do not contain sub tokens. These are most of the time operators, names, values...
    ///
    #[derive(Debug)]
    pub struct TerminalToken {
        pub span: (usize, usize),
        pub rule: ast::Rule,
        pub content: std::string::String,
    }

    ///
    /// Root Token: Simple wrapper containing the source that has been parsed and the resulting tokens.
    ///
    #[derive(Debug)]
    pub struct RootToken {
        pub source: std::string::String,
        pub ast: Vec<Token>
    }
}

#[derive(Debug)]
pub enum ParseError {
    IOError(std::io::Error),
    PestError(pest::error::Error<Rule>)
}

///
/// Representation of a file. Two main characs, its AST and the other files it includes.
///
#[derive(Debug)]
pub struct File {

    pub path: std::path::PathBuf,
    pub root: tokens::RootToken,
    pub includes: Vec<std::path::PathBuf>

}

///
/// Converts the pest AST structure into the snarkrs AST structure. The pest AST is then consumed.
///
pub fn pairs_to_tokens(pairs: Vec<pest::iterators::Pair<Rule>>) -> Vec<tokens::Token> {

    let mut return_value: Vec<tokens::Token> = Vec::new();

    for pair in pairs.into_iter() {

        let rule: Rule = pair.as_rule();
        let span: pest::Span = pair.as_span();
        let inner_pairs: Vec<pest::iterators::Pair<Rule>> = pair.into_inner().into_iter().collect();

        match inner_pairs.len() {

            0 => {
                return_value.push(tokens::Token::Terminal(tokens::TerminalToken {
                    span: (span.start(), span.end()),
                    rule,
                    content: span.as_str().to_string()
                }));
            },

            _ => {
                return_value.push(tokens::Token::NonTerminal(tokens::NonTerminalToken {
                    span: (span.start(), span.end()),
                    rule,
                    subrules: pairs_to_tokens(inner_pairs)
                }));
            }

        }
    };

    return return_value;

}

///
/// Converts the source and the pest AST into a RootToken. Calls `pairs_to_tokens`.
///
pub fn pest_to_tokens(source: & str, pairs: pest::iterators::Pairs<Rule>) -> tokens::RootToken {
    tokens::RootToken {
        source: source.to_string(),
        ast: pairs_to_tokens(pairs.into_iter().collect())
    }
}

///
/// Takes a source buffer as input and returns the converted AST.
///
pub fn parse_source(sources: & str, maybe_rule: Option<Rule>) -> std::result::Result<tokens::RootToken, pest::error::Error<Rule>> {
    match

        if let Some(rule) = maybe_rule {
            CircuitParser::parse(rule, sources)
        } else {
            CircuitParser::parse(Rule::Circuit, sources)
        }
        {
            Ok(ast) => Ok(pest_to_tokens(sources, ast)),
            Err(error) => Err(error)
        }
}

///
/// Takes a path, loads it into memory, run the parsing and return the built `File` type.
///
pub fn parse_file(path: & std::path::PathBuf) -> Result<File, ParseError> {

    let path = std::path::PathBuf::from(path);
    let content = match std::fs::read_to_string(&path) {
        Ok(val) => val,
        Err(error) => return Err(ParseError::IOError(error))
    };

    Ok(File {
        path,
        root: match parse_source(&content, None) {
            Ok(val) => val,
            Err(error) => return Err(ParseError::PestError(error))
        },
        includes: Vec::new()
    })

}

#[cfg(test)]
mod parser_tests {

    use galvanic_assert::matchers::*;

    use parser::ast;
    use pest::error::{LineColLocation, ErrorVariant};
    use parser::ast::{parse_file};

    fn test_untupler(expect_rules_values: &Vec<(ast::Rule, u32)>) -> Vec<ast::Rule> {

        let mut ret: Vec<ast::Rule> = Vec::new();

        for rule_idx in 0..expect_rules_values.len() {
            ret.push(
                expect_rules_values[rule_idx].0
            );
        }

        ret
    }

    fn count_rules(token: & ast::tokens::Token, needed_rules: &Vec<ast::Rule>) -> Vec<u32> {

        match token {
            ast::tokens::Token::Terminal(terminal) => {

                let mut single: Vec<u32> = Vec::new();

                for rule in needed_rules {
                    if *rule == terminal.rule {
                        single.push(1);
                    } else {
                        single.push(0);
                    }
                }

                single
            },
            ast::tokens::Token::NonTerminal(nterminal) => {

                let mut single: Vec<u32> = Vec::new();

                for rule in needed_rules {
                    if *rule == nterminal.rule {
                        single.push(1);
                    } else {
                        single.push(0);
                    }
                }

                for sub_rule in &nterminal.subrules {
                    let inner_ret = count_rules(sub_rule, needed_rules);
                    for idx in 0..single.len() {
                        single[idx] += inner_ret[idx];
                    }
                }

                single

            }
        }
    }

    #[test]
    fn complete_parse() {

        let pathbuf: std::path::PathBuf = match std::fs::canonicalize("./src/lib/parser/test_material/circuits/bitify.circom") {
            Err(error) => panic!(error),
            Ok(path) => path
        };

        parse_file(&pathbuf).expect("Could not parse file");

    }

    //
    //
    //

    #[test]
    fn valid_schemas() {
        let filenames: Vec<&str> = vec!(
            "./src/lib/parser/test_material/include/valid_include.circom",
            "./src/lib/parser/test_material/functions/valid_function.circom",
            "./src/lib/parser/test_material/templates/valid_template.circom"
        );

        for filename in filenames {

            let contents = std::fs::read_to_string(filename).expect(&format!("Cannot read file {}", filename));

            match ast::parse_source(&contents, Some(ast::Rule::Circuit)) {
                Ok(_pairs) => {},
                Err(err) => panic!(err)
            }

        }
    }

    //
    // Include testing
    //

    #[test]
    fn include_it_fails_on_missing_semicolon() {
        let filename: &str = "./src/lib/parser/test_material/include/invalid_include__missing_semicolon.circom";
        let contents = std::fs::read_to_string(filename).expect(&format!("Cannot read file {}", filename));
        match ast::parse_source(&contents, Some(ast::Rule::IncludeStatement)) {
            Ok(_pairs) => {},
            Err(err) => {

                assert_eq!(err.line_col, LineColLocation::Pos((1, 34)));
                assert_eq!(err.variant, ErrorVariant::ParsingError {positives: vec!(ast::Rule::COMMENT, ast::Rule::END_OF_LINE), negatives: vec!()});

            }
        }
    }

    #[test]
    fn include_it_fails_on_directive_typo() {
        let filename: &str = "./src/lib/parser/test_material/include/invalid_include__directive_typo.circom";
        let contents = std::fs::read_to_string(filename).expect(&format!("Cannot read file {}", filename));
        match ast::parse_source(&contents, Some(ast::Rule::IncludeStatement)) {
            Ok(_pairs) => {},
            Err(err) => {

                assert_eq!(err.line_col, LineColLocation::Pos((1, 1)));
                assert_eq!(err.variant, ErrorVariant::ParsingError {positives: vec!(ast::Rule::IncludeKW), negatives: vec!()});

            }
        }
    }

    #[test]
    fn include_it_fails_on_empty_value() {
        let filename: &str = "./src/lib/parser/test_material/include/invalid_include__empty_include.circom";
        let contents = std::fs::read_to_string(filename).expect(&format!("Cannot read file {}", filename));
        match ast::parse_source(&contents, Some(ast::Rule::IncludeStatement)) {
            Ok(_pairs) => {},
            Err(err) => {

                assert_eq!(err.line_col, LineColLocation::Pos((1, 10)));
                assert_eq!(err.variant, ErrorVariant::ParsingError {positives: vec!(ast::Rule::FilesystemPath), negatives: vec!()});

            }
        }
    }

    //
    // Function testing
    //

    #[test]
    fn function_invalid_missing_semicolon() {
        let filename: &str = "./src/lib/parser/test_material/functions/invalid_function__missing_semicolon.circom";
        let contents = std::fs::read_to_string(filename).expect(&format!("Cannot read file {}", filename));
        match ast::parse_source(&contents, Some(ast::Rule::FunctionBlock)) {
            Ok(_pairs) => {},
            Err(err) => {

                assert_eq!(err.line_col, LineColLocation::Pos((3, 1)));

            }
        }
    }

    #[test]
    fn expression_valid() {
        let filenames: Vec<&str> = vec!(
            "./src/lib/parser/test_material/expressions/valid_terminal_values.circom",
            "./src/lib/parser/test_material/expressions/valid_array_of_expressions.circom",
            "./src/lib/parser/test_material/expressions/valid_brackets_expressions.circom",
            "./src/lib/parser/test_material/expressions/valid_signal_operations.circom",
            "./src/lib/parser/test_material/expressions/valid_ternary_operations.circom",
            "./src/lib/parser/test_material/expressions/valid_logical_operations.circom",
            "./src/lib/parser/test_material/expressions/valid_bitwise_operations.circom",
            "./src/lib/parser/test_material/expressions/valid_relational_equality_operations.circom",
            "./src/lib/parser/test_material/expressions/valid_relational_ordering_operations.circom",
            "./src/lib/parser/test_material/expressions/valid_bitwise_shift_operations.circom",
            "./src/lib/parser/test_material/expressions/valid_sum_operations.circom",
            "./src/lib/parser/test_material/expressions/valid_product_operations.circom",
            "./src/lib/parser/test_material/expressions/valid_exponential_operations.circom",
            "./src/lib/parser/test_material/expressions/valid_prefix_operations.circom",
            "./src/lib/parser/test_material/expressions/valid_postfix_operations.circom",
            "./src/lib/parser/test_material/expressions/valid_member_access_operations.circom",
            "./src/lib/parser/test_material/expressions/valid_braced_operations.circom"
        );

        let expected_rules: Vec<Vec<(ast::Rule, u32)>> = vec!(
            vec!(
                (ast::Rule::E_Decimal, 1),
                (ast::Rule::E_Hexadecimal, 1),
                (ast::Rule::E_VariableName, 1),
                (ast::Rule::Expression, 3)
            ),

            vec!(
                (ast::Rule::E_0_CommaOperator, 1),
                (ast::Rule::E_Array, 3),
                (ast::Rule::Expression, 5)
            ),

            vec!(
                (ast::Rule::E_0_CommaOperator, 2),
                (ast::Rule::E_Brackets, 3),
                (ast::Rule::Expression, 4)
            ),

            vec!(
                (ast::Rule::E_0_CommaOperator, 0),
                (ast::Rule::E_Brackets, 25),
                (ast::Rule::E_1_SignalAssertionConstraintOperator, 6),
                (ast::Rule::E_18_PostfixOperator, 0),
                (ast::Rule::E_2_SignalLeftHandOperator, 12),
                (ast::Rule::E_3_SignalRightHandOperator, 12),
                (ast::Rule::Expression, 50)
            ),

            vec!(
                (ast::Rule::E_0_CommaOperator, 0),
                (ast::Rule::E_Brackets, 1),
                (ast::Rule::E_5_TernaryFirstOperator, 4),
                (ast::Rule::Expression, 4)
            ),

            vec!(
                (ast::Rule::E_0_CommaOperator, 0),
                (ast::Rule::E_Brackets, 1),
                (ast::Rule::E_6_LogicalOrOperator, 3),
                (ast::Rule::E_7_LogicalAndOperator, 3),
                (ast::Rule::Expression, 5)
            ),

            vec!(
                (ast::Rule::E_0_CommaOperator, 0),
                (ast::Rule::E_Brackets, 3),
                (ast::Rule::E_8_BitwiseOrOperator, 4),
                (ast::Rule::E_9_BitwiseXorOperator, 3),
                (ast::Rule::E_10_BitwiseAndOperator, 5),
                (ast::Rule::Expression, 10)
            ),

            vec!(
                (ast::Rule::E_0_CommaOperator, 0),
                (ast::Rule::E_Brackets, 2),
                (ast::Rule::E_11_RelationalEqualityOperator, 6),
                (ast::Rule::Expression, 5)
            ),

            vec!(
                (ast::Rule::E_0_CommaOperator, 0),
                (ast::Rule::E_Brackets, 3),
                (ast::Rule::E_12_RelationalOrderingOperator, 8),
                (ast::Rule::Expression, 8)
            ),

            vec!(
                (ast::Rule::E_0_CommaOperator, 0),
                (ast::Rule::E_Brackets, 1),
                (ast::Rule::E_13_BitwiseShiftOperator, 6),
                (ast::Rule::Expression, 5)
            ),

            vec!(
                (ast::Rule::E_0_CommaOperator, 0),
                (ast::Rule::E_Brackets, 5),
                (ast::Rule::E_14_SumOperator, 13),
                (ast::Rule::Expression, 12)
            ),

            vec!(
                (ast::Rule::E_0_CommaOperator, 0),
                (ast::Rule::E_Brackets, 2),
                (ast::Rule::E_15_ProductOperator, 8),
                (ast::Rule::Expression, 7)
            ),

            vec!(
                (ast::Rule::E_0_CommaOperator, 0),
                (ast::Rule::E_Brackets, 4),
                (ast::Rule::E_16_ExponentialOperator, 4),
                (ast::Rule::Expression, 8)
            ),

            vec!(
                (ast::Rule::E_0_CommaOperator, 0),
                (ast::Rule::E_Brackets, 4),
                (ast::Rule::E_17_PrefixOperator, 13),
                (ast::Rule::Expression, 12)
            ),

            vec!(
                (ast::Rule::E_0_CommaOperator, 0),
                (ast::Rule::E_Brackets, 4),
                (ast::Rule::E_18_PostfixOperator, 5),
                (ast::Rule::Expression, 7)
            ),

            vec!(
                (ast::Rule::E_0_CommaOperator, 0),
                (ast::Rule::E_Brackets, 1),
                (ast::Rule::E_17_PrefixOperator, 1),
                (ast::Rule::E_19_MemberAccessOperator, 6),
                (ast::Rule::Expression, 5)
            ),

            vec!(
                (ast::Rule::E_0_CommaOperator, 1),
                (ast::Rule::E_Brackets, 0),
                (ast::Rule::E_20_BracedOperatorOpen, 8),
                (ast::Rule::Expression, 10)
            ),

        );

        for test_idx in 0..filenames.len() {

            let contents = std::fs::read_to_string(filenames[test_idx]).expect(&format!("Cannot read file {}", filenames[test_idx]));
            match ast::parse_source(& contents, Some(ast::Rule::Body)) {
                Ok(tokens) => {

                    println!("----------");
                    println!("{}\n", filenames[test_idx]);

                    let res: Vec<u32> = count_rules(&tokens.ast[0], &test_untupler(&expected_rules[test_idx]));

                    for check_idx in 0..expected_rules[test_idx].len() {

                        let assert_res = std::panic::catch_unwind(|| {
                            assert_eq!(res[check_idx], expected_rules[test_idx][check_idx].1)
                        });

                        match assert_res {
                            Err(_) => {
                                panic!("Found {} Rule::{:?}, expected {}", res[check_idx], expected_rules[test_idx][check_idx].0, expected_rules[test_idx][check_idx].1);
                            },
                            Ok(_) => {
                                println!("Successfully found {} Rule::{:?}", res[check_idx], expected_rules[test_idx][check_idx].0);
                            }
                        }
                    }
                    println!("----------");

                },
                Err(err) => panic!(err)
            }

        }
    }

    //
    // Circuits testing
    //

    #[test]
    fn valid_circuits() {
        let filenames: Vec<&str> = vec!(
            "./src/lib/parser/test_material/circuits/aliascheck.circom",
            "./src/lib/parser/test_material/circuits/babyjub.circom",
            "./src/lib/parser/test_material/circuits/binsub.circom",
            "./src/lib/parser/test_material/circuits/binsum.circom",
            "./src/lib/parser/test_material/circuits/bitify.circom",
            "./src/lib/parser/test_material/circuits/comparators.circom",
            "./src/lib/parser/test_material/circuits/compconstant.circom",
            "./src/lib/parser/test_material/circuits/eddsa.circom",
            "./src/lib/parser/test_material/circuits/eddsamimc.circom",
            "./src/lib/parser/test_material/circuits/escalarmul.circom",
            "./src/lib/parser/test_material/circuits/escalarmulany.circom",
            "./src/lib/parser/test_material/circuits/escalarmulfix.circom",
            "./src/lib/parser/test_material/circuits/escalarmulw4table.circom",
            "./src/lib/parser/test_material/circuits/gates.circom",
            "./src/lib/parser/test_material/circuits/mimc.circom",
            "./src/lib/parser/test_material/circuits/montgomery.circom",
            "./src/lib/parser/test_material/circuits/multiplexer.circom",
            "./src/lib/parser/test_material/circuits/mux3.circom",
            "./src/lib/parser/test_material/circuits/mux4.circom",
            "./src/lib/parser/test_material/circuits/pedersen.circom",
            "./src/lib/parser/test_material/circuits/pedersen_old.circom",
            "./src/lib/parser/test_material/circuits/pointbits.circom",
            "./src/lib/parser/test_material/circuits/sign.circom",
            "./src/lib/parser/test_material/circuits/switcher.circom",
        );

        let filechecks: Vec<Vec<(ast::Rule, u32)>> = vec!(

            vec!( // aliascheck.circom
                  (ast::Rule::IncludeStatement, 1),
                  (ast::Rule::TemplateBlock, 1),
                  (ast::Rule::DeclarationStatement, 3),
                  (ast::Rule::ForStatement, 1),
                  (ast::Rule::E_1_SignalAssertionConstraintOperator, 1)
            ),

            vec!( // babyjub.circom
                  (ast::Rule::IncludeStatement, 0),
                  (ast::Rule::TemplateBlock, 3),
                  (ast::Rule::DeclarationStatement, 23),
                  (ast::Rule::ForStatement, 0),
                  (ast::Rule::E_1_SignalAssertionConstraintOperator, 3),
                  (ast::Rule::E_2_SignalLeftHandOperator, 12),
                  (ast::Rule::E_3_SignalRightHandOperator, 2)
            ),

            vec!( // binsub.circom
                  (ast::Rule::IncludeStatement, 0),
                  (ast::Rule::TemplateBlock, 1),
                  (ast::Rule::DeclarationStatement, 7),
                  (ast::Rule::ForStatement, 2),
                  (ast::Rule::E_1_SignalAssertionConstraintOperator, 3),
                  (ast::Rule::E_2_SignalLeftHandOperator, 2),
                  (ast::Rule::E_3_SignalRightHandOperator, 0)
            ),

            vec!( // binsum.circom
                  (ast::Rule::IncludeStatement, 0),
                  (ast::Rule::TemplateBlock, 1),
                  (ast::Rule::FunctionBlock, 1),
                  (ast::Rule::ReturnStatement, 1),
                  (ast::Rule::DeclarationStatement, 9),
                  (ast::Rule::ForStatement, 3),
                  (ast::Rule::E_1_SignalAssertionConstraintOperator, 2),
                  (ast::Rule::E_2_SignalLeftHandOperator, 1),
                  (ast::Rule::E_3_SignalRightHandOperator, 0)
            ),

            vec!( // bitify.circom
                  (ast::Rule::IncludeStatement, 2),
                  (ast::Rule::TemplateBlock, 5),
                  (ast::Rule::FunctionBlock, 0),
                  (ast::Rule::ReturnStatement, 0),
                  (ast::Rule::DeclarationStatement, 24),
                  (ast::Rule::ForStatement, 5),
                  (ast::Rule::E_1_SignalAssertionConstraintOperator, 4),
                  (ast::Rule::E_2_SignalLeftHandOperator, 2),
                  (ast::Rule::E_3_SignalRightHandOperator, 8)
            ),

            vec!( // comparators.circom
                  (ast::Rule::IncludeStatement, 2),
                  (ast::Rule::TemplateBlock, 4),
                  (ast::Rule::FunctionBlock, 0),
                  (ast::Rule::ReturnStatement, 0),
                  (ast::Rule::DeclarationStatement, 15),
                  (ast::Rule::ForStatement, 1),
                  (ast::Rule::E_1_SignalAssertionConstraintOperator, 2),
                  (ast::Rule::E_2_SignalLeftHandOperator, 2),
                  (ast::Rule::E_3_SignalRightHandOperator, 8)
            ),

            vec!( // compconstant.circom
                  (ast::Rule::IncludeStatement, 1),
                  (ast::Rule::TemplateBlock, 1),
                  (ast::Rule::FunctionBlock, 0),
                  (ast::Rule::ReturnStatement, 0),
                  (ast::Rule::DeclarationStatement, 14),
                  (ast::Rule::IfStatement, 1),
                  (ast::Rule::ForStatement, 1),
                  (ast::Rule::E_1_SignalAssertionConstraintOperator, 0),
                  (ast::Rule::E_2_SignalLeftHandOperator, 7),
                  (ast::Rule::E_3_SignalRightHandOperator, 0)
            ),

            vec!( // eddsa.circom
                  (ast::Rule::IncludeStatement, 5),
                  (ast::Rule::TemplateBlock, 1),
                  (ast::Rule::FunctionBlock, 0),
                  (ast::Rule::ReturnStatement, 0),
                  (ast::Rule::DeclarationStatement, 22),
                  (ast::Rule::IfStatement, 0),
                  (ast::Rule::ForStatement, 7),
                  (ast::Rule::E_1_SignalAssertionConstraintOperator, 6),
                  (ast::Rule::E_2_SignalLeftHandOperator, 26),
                  (ast::Rule::E_3_SignalRightHandOperator, 1)
            ),

            vec!( // eddsamimc.circom
                  (ast::Rule::IncludeStatement, 6),
                  (ast::Rule::TemplateBlock, 1),
                  (ast::Rule::FunctionBlock, 0),
                  (ast::Rule::ReturnStatement, 0),
                  (ast::Rule::DeclarationStatement, 22),
                  (ast::Rule::IfStatement, 0),
                  (ast::Rule::ForStatement, 3),
                  (ast::Rule::E_1_SignalAssertionConstraintOperator, 2),
                  (ast::Rule::E_2_SignalLeftHandOperator, 29),
                  (ast::Rule::E_3_SignalRightHandOperator, 1)
            ),

            vec!( // escalarmul.circom
                  (ast::Rule::IncludeStatement, 3),
                  (ast::Rule::TemplateBlock, 2),
                  (ast::Rule::FunctionBlock, 0),
                  (ast::Rule::ReturnStatement, 0),
                  (ast::Rule::DeclarationStatement, 14),
                  (ast::Rule::IfStatement, 1),
                  (ast::Rule::ForStatement, 6),
                  (ast::Rule::E_1_SignalAssertionConstraintOperator, 0),
                  (ast::Rule::E_2_SignalLeftHandOperator, 4),
                  (ast::Rule::E_3_SignalRightHandOperator, 13)
            ),

            vec!( // escalarmulany.circom
                  (ast::Rule::IncludeStatement, 2),
                  (ast::Rule::TemplateBlock, 4),
                  (ast::Rule::FunctionBlock, 0),
                  (ast::Rule::ReturnStatement, 0),
                  (ast::Rule::DeclarationStatement, 33),
                  (ast::Rule::IfStatement, 3),
                  (ast::Rule::ForStatement, 3),
                  (ast::Rule::E_1_SignalAssertionConstraintOperator, 0),
                  (ast::Rule::E_2_SignalLeftHandOperator, 2),
                  (ast::Rule::E_3_SignalRightHandOperator, 61)
            ),

            vec!( // escalarmulfix.circom
                  (ast::Rule::IncludeStatement, 3),
                  (ast::Rule::TemplateBlock, 3),
                  (ast::Rule::FunctionBlock, 0),
                  (ast::Rule::ReturnStatement, 0),
                  (ast::Rule::DeclarationStatement, 36),
                  (ast::Rule::IfStatement, 6),
                  (ast::Rule::ForStatement, 5),
                  (ast::Rule::E_1_SignalAssertionConstraintOperator, 0),
                  (ast::Rule::E_2_SignalLeftHandOperator, 84),
                  (ast::Rule::E_3_SignalRightHandOperator, 18)
            ),

            vec!( // escalarmulw4table.circom
                  (ast::Rule::IncludeStatement, 0),
                  (ast::Rule::TemplateBlock, 1),
                  (ast::Rule::FunctionBlock, 1),
                  (ast::Rule::ReturnStatement, 1),
                  (ast::Rule::DeclarationStatement, 7),
                  (ast::Rule::IfStatement, 0),
                  (ast::Rule::ForStatement, 2),
                  (ast::Rule::E_1_SignalAssertionConstraintOperator, 0),
                  (ast::Rule::E_2_SignalLeftHandOperator, 4),
                  (ast::Rule::E_3_SignalRightHandOperator, 0)
            ),

            vec!( // gates.circom
                  (ast::Rule::IncludeStatement, 0),
                  (ast::Rule::TemplateBlock, 7),
                  (ast::Rule::FunctionBlock, 0),
                  (ast::Rule::ReturnStatement, 0),
                  (ast::Rule::DeclarationStatement, 26),
                  (ast::Rule::IfStatement, 1),
                  (ast::Rule::ForStatement, 2),
                  (ast::Rule::E_1_SignalAssertionConstraintOperator, 0),
                  (ast::Rule::E_2_SignalLeftHandOperator, 15),
                  (ast::Rule::E_3_SignalRightHandOperator, 0)
            ),

            vec!( // mimc.circom
                  (ast::Rule::IncludeStatement, 0),
                  (ast::Rule::TemplateBlock, 2),
                  (ast::Rule::FunctionBlock, 0),
                  (ast::Rule::ReturnStatement, 0),
                  (ast::Rule::DeclarationStatement, 14),
                  (ast::Rule::IfStatement, 2),
                  (ast::Rule::ForStatement, 2),
                  (ast::Rule::E_1_SignalAssertionConstraintOperator, 0),
                  (ast::Rule::E_2_SignalLeftHandOperator, 9),
                  (ast::Rule::E_3_SignalRightHandOperator, 0)
            ),

            vec!( // montgomery.circom
                  (ast::Rule::IncludeStatement, 0),
                  (ast::Rule::TemplateBlock, 4),
                  (ast::Rule::FunctionBlock, 0),
                  (ast::Rule::ReturnStatement, 0),
                  (ast::Rule::DeclarationStatement, 20),
                  (ast::Rule::IfStatement, 0),
                  (ast::Rule::ForStatement, 0),
                  (ast::Rule::E_1_SignalAssertionConstraintOperator, 6),
                  (ast::Rule::E_2_SignalLeftHandOperator, 11),
                  (ast::Rule::E_3_SignalRightHandOperator, 0)
            ),

            vec!( // multiplexer.circom
                  (ast::Rule::IncludeStatement, 0),
                  (ast::Rule::TemplateBlock, 3),
                  (ast::Rule::FunctionBlock, 0),
                  (ast::Rule::ReturnStatement, 0),
                  (ast::Rule::DeclarationStatement, 19),
                  (ast::Rule::IfStatement, 0),
                  (ast::Rule::ForStatement, 4),
                  (ast::Rule::E_1_SignalAssertionConstraintOperator, 3),
                  (ast::Rule::E_2_SignalLeftHandOperator, 3),
                  (ast::Rule::E_3_SignalRightHandOperator, 5)
            ),

            vec!( // mux3.circom
                  (ast::Rule::IncludeStatement, 0),
                  (ast::Rule::TemplateBlock, 2),
                  (ast::Rule::FunctionBlock, 0),
                  (ast::Rule::ReturnStatement, 0),
                  (ast::Rule::DeclarationStatement, 18),
                  (ast::Rule::IfStatement, 0),
                  (ast::Rule::ForStatement, 3),
                  (ast::Rule::E_1_SignalAssertionConstraintOperator, 0),
                  (ast::Rule::E_2_SignalLeftHandOperator, 11),
                  (ast::Rule::E_3_SignalRightHandOperator, 2)
            ),

            vec!( // mux4.circom
                  (ast::Rule::IncludeStatement, 0),
                  (ast::Rule::TemplateBlock, 2),
                  (ast::Rule::FunctionBlock, 0),
                  (ast::Rule::ReturnStatement, 0),
                  (ast::Rule::DeclarationStatement, 29),
                  (ast::Rule::IfStatement, 0),
                  (ast::Rule::ForStatement, 3),
                  (ast::Rule::E_1_SignalAssertionConstraintOperator, 0),
                  (ast::Rule::E_2_SignalLeftHandOperator, 22),
                  (ast::Rule::E_3_SignalRightHandOperator, 2)
            ),

            vec!( // pedersen.circom
                  (ast::Rule::IncludeStatement, 3),
                  (ast::Rule::TemplateBlock, 3),
                  (ast::Rule::FunctionBlock, 0),
                  (ast::Rule::ReturnStatement, 0),
                  (ast::Rule::DeclarationStatement, 33),
                  (ast::Rule::IfStatement, 5),
                  (ast::Rule::ForStatement, 6),
                  (ast::Rule::E_1_SignalAssertionConstraintOperator, 0),
                  (ast::Rule::E_2_SignalLeftHandOperator, 88),
                  (ast::Rule::E_3_SignalRightHandOperator, 0)
            ),

            vec!( // pedersen_old.circom
                  (ast::Rule::IncludeStatement, 1),
                  (ast::Rule::TemplateBlock, 1),
                  (ast::Rule::FunctionBlock, 0),
                  (ast::Rule::ReturnStatement, 0),
                  (ast::Rule::DeclarationStatement, 9),
                  (ast::Rule::IfStatement, 1),
                  (ast::Rule::ForStatement, 2),
                  (ast::Rule::E_1_SignalAssertionConstraintOperator, 0),
                  (ast::Rule::E_2_SignalLeftHandOperator, 5),
                  (ast::Rule::E_3_SignalRightHandOperator, 2)
            ),

            vec!( // pointbits.circom
                  (ast::Rule::IncludeStatement, 4),
                  (ast::Rule::TemplateBlock, 4),
                  (ast::Rule::FunctionBlock, 1),
                  (ast::Rule::ReturnStatement, 3),
                  (ast::Rule::DeclarationStatement, 34),
                  (ast::Rule::IfStatement, 4),
                  (ast::Rule::ForStatement, 8),
                  (ast::Rule::WhileStatement, 2),
                  (ast::Rule::E_1_SignalAssertionConstraintOperator, 2),
                  (ast::Rule::E_2_SignalLeftHandOperator, 17),
                  (ast::Rule::E_3_SignalRightHandOperator, 0)
            ),

            vec!( // sign.circom
                  (ast::Rule::IncludeStatement, 1),
                  (ast::Rule::TemplateBlock, 1),
                  (ast::Rule::FunctionBlock, 0),
                  (ast::Rule::ReturnStatement, 0),
                  (ast::Rule::DeclarationStatement, 4),
                  (ast::Rule::IfStatement, 0),
                  (ast::Rule::ForStatement, 1),
                  (ast::Rule::WhileStatement, 0),
                  (ast::Rule::E_1_SignalAssertionConstraintOperator, 0),
                  (ast::Rule::E_2_SignalLeftHandOperator, 2),
                  (ast::Rule::E_3_SignalRightHandOperator, 0)
            ),

            vec!( // switcher.circom
                  (ast::Rule::IncludeStatement, 0),
                  (ast::Rule::TemplateBlock, 1),
                  (ast::Rule::FunctionBlock, 0),
                  (ast::Rule::ReturnStatement, 0),
                  (ast::Rule::DeclarationStatement, 6),
                  (ast::Rule::IfStatement, 0),
                  (ast::Rule::ForStatement, 0),
                  (ast::Rule::WhileStatement, 0),
                  (ast::Rule::E_1_SignalAssertionConstraintOperator, 0),
                  (ast::Rule::E_2_SignalLeftHandOperator, 3),
                  (ast::Rule::E_3_SignalRightHandOperator, 0)
            )

        );

        for idx in 0..filenames.len() {

            let contents = std::fs::read_to_string(filenames[idx]).expect(&format!("Cannot read file {}", filenames[idx]));
            match ast::parse_source(& contents, Some(ast::Rule::Circuit)) {
                Ok(tokens) => {

                    println!("----------");
                    println!("{}\n", filenames[idx]);

                    let res: Vec<u32> = count_rules(&tokens.ast[0], &test_untupler(&filechecks[idx]));

                    for check_idx in 0..filechecks[idx].len() {

                        let assert_res = std::panic::catch_unwind(|| {
                            assert_eq!(res[check_idx], filechecks[idx][check_idx].1)
                        });

                        match assert_res {
                            Err(_) => {
                                panic!("Found {} Rule::{:?}, expected {}", res[check_idx], filechecks[idx][check_idx].0, filechecks[idx][check_idx].1);
                            },
                            Ok(_) => {
                                println!("Successfully found {} Rule::{:?}", res[check_idx], filechecks[idx][check_idx].0);
                            }
                        }
                    }
                    println!("----------");
                },
                Err(err) => panic!(err)
            }

        }
    }

}

