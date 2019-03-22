use pest::Parser;

#[derive(Parser)]
#[grammar = "lib/parser/grammar.pest"]
struct CircuitParser;

pub fn recurse_down(pairs: pest::iterators::Pairs<Rule>, depth: u32) {

    let sub_pairs = pairs.into_iter();

    for sub_pair in sub_pairs {
        println!("Rule: {:?}, Content: {:?} at Height {:?}", sub_pair.as_rule(), sub_pair.as_str(), depth);
        recurse_down(sub_pair.into_inner(), depth + 1);
    }
}

pub fn parse(sources: &str, maybe_rule: Option<Rule>) -> std::result::Result<pest::iterators::Pairs<Rule>, pest::error::Error<Rule>> {
    if let Some(rule) = maybe_rule {
        CircuitParser::parse(rule, sources)
    } else {
        CircuitParser::parse(Rule::IncludeStatement, sources)
    }
}

#[cfg(test)]
mod parser_tests {

    use galvanic_assert::matchers::*;

    use parser::ast;
    use pest::error::{LineColLocation, InputLocation, ErrorVariant};

    #[test]
    fn include_it_parses_correctly() {
        let filename: &str = "./src/lib/parser/test_material/include/valid_include.circom";
        let contents = std::fs::read_to_string(filename).expect(&format!("Cannot read file {}", filename));
        match ast::parse(&contents, Some(ast::Rule::IncludeStatement)) {
            Ok(pairs) => {

                let tokens: Vec<_> = pairs.flatten().into_iter().collect();

                assert_that!(&tokens.len(), eq(5));
                assert_that!(&tokens[0].as_str(), eq("include \"my_other_circuit.circom\" ;"));
                assert_that!(&tokens[1].as_str(), eq("include "));
                assert_that!(&tokens[2].as_str(), eq("\"my_other_circuit.circom\""));
                assert_that!(&tokens[3].as_str(), eq("my_other_circuit.circom"));
                assert_that!(&tokens[4].as_str(), eq(";"));

            },
            Err(err) => panic!(err)
        }
    }

    #[test]
    fn include_it_fails_on_missing_semicolon() {
        let filename: &str = "./src/lib/parser/test_material/include/invalid_include__missing_semicolon.circom";
        let contents = std::fs::read_to_string(filename).expect(&format!("Cannot read file {}", filename));
        match ast::parse(&contents, Some(ast::Rule::IncludeStatement)) {
            Ok(_pairs) => {},
            Err(err) => {

                assert_eq!(err.line_col, LineColLocation::Pos((1, 34)));
                assert_eq!(err.location, InputLocation::Pos(33));
                assert_eq!(err.variant, ErrorVariant::ParsingError {positives: vec!(ast::Rule::end_of_line), negatives: vec!()});

            }
        }
    }

    #[test]
    fn include_it_fails_on_directive_typo() {
        let filename: &str = "./src/lib/parser/test_material/include/invalid_include__directive_typo.circom";
        let contents = std::fs::read_to_string(filename).expect(&format!("Cannot read file {}", filename));
        match ast::parse(&contents, Some(ast::Rule::IncludeStatement)) {
            Ok(_pairs) => {},
            Err(err) => {

                assert_eq!(err.line_col, LineColLocation::Pos((1, 1)));
                assert_eq!(err.location, InputLocation::Pos(0));
                assert_eq!(err.variant, ErrorVariant::ParsingError {positives: vec!(ast::Rule::IncludeKW), negatives: vec!()});

            }
        }
    }

    #[test]
    fn include_it_fails_on_empty_value() {
        let filename: &str = "./src/lib/parser/test_material/include/invalid_include__empty_include.circom";
        let contents = std::fs::read_to_string(filename).expect(&format!("Cannot read file {}", filename));
        match ast::parse(&contents, Some(ast::Rule::IncludeStatement)) {
            Ok(_pairs) => {},
            Err(err) => {

                assert_eq!(err.line_col, LineColLocation::Pos((1, 10)));
                assert_eq!(err.location, InputLocation::Pos(9));
                assert_eq!(err.variant, ErrorVariant::ParsingError {positives: vec!(ast::Rule::FilesystemPath), negatives: vec!()});

            }
        }
    }

    #[test]
    fn template_valid() {
        let filename: &str = "./src/lib/parser/test_material/include/valid_template.circom";
        let contents = std::fs::read_to_string(filename).expect(&format!("Cannot read file {}", filename));
        match ast::parse(&contents, Some(ast::Rule::TemplateBlock)) {
            Ok(pairs) => {
                ast::recurse_down(pairs, 0)
            },
            Err(err) => {

                assert_eq!(err.variant, ErrorVariant::ParsingError {positives: vec!(ast::Rule::FilesystemPath), negatives: vec!()});
                assert_eq!(err.line_col, LineColLocation::Pos((1, 10)));
                assert_eq!(err.location, InputLocation::Pos(9));

            }
        }
    }

    #[test]
    fn function_valid() {
        let filename: &str = "./src/lib/parser/test_material/include/valid_function.circom";
        let contents = std::fs::read_to_string(filename).expect(&format!("Cannot read file {}", filename));
        match ast::parse(&contents, Some(ast::Rule::FunctionBlock)) {
            Ok(pairs) => {
                ast::recurse_down(pairs, 0)
            },
            Err(err) => {

                assert_eq!(err.variant, ErrorVariant::ParsingError {positives: vec!(ast::Rule::FilesystemPath), negatives: vec!()});
                assert_eq!(err.line_col, LineColLocation::Pos((1, 10)));
                assert_eq!(err.location, InputLocation::Pos(9));

            }
        }
    }

    #[test]
    fn function_invalid_missing_semicolon() {
        let filename: &str = "./src/lib/parser/test_material/include/invalid_function__missing_semicolon.circom";
        let contents = std::fs::read_to_string(filename).expect(&format!("Cannot read file {}", filename));
        match ast::parse(&contents, Some(ast::Rule::FunctionBlock)) {
            Ok(pairs) => {
                ast::recurse_down(pairs, 0)
            },
            Err(err) => {

                assert_eq!(err.line_col, LineColLocation::Pos((3, 1)));
                assert_eq!(err.location, InputLocation::Pos(35));

            }
        }
    }

}

