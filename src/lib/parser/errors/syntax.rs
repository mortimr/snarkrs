use std::fmt;
use parser::errors::common_displayer;

///
/// E101: Syntax Error
///
/// An Syntaxic Error, discovered by the pest parser.
///
#[derive(Debug, Clone)]
pub struct SyntaxError {
    pub file_error: std::path::PathBuf,
    pub source_error: std::string::String,
    pub span_error: (usize, usize),

}

impl fmt::Display for SyntaxError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {

        common_displayer::common_displayer(formatter, "SyntaxError", &101, &self.source_error, &self.span_error, &self.file_error)

    }
}
