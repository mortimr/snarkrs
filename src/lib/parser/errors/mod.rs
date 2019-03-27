pub mod include;
pub mod syntax;
pub mod common_displayer;

use parser::ast::ParseError;
use parser::errors::include::UnknownFileIncludeError;

#[derive(Debug)]
pub enum CompileError {
    UnknwonFileIncludeError(include::UnknownFileIncludeError),
    SyntaxError(syntax::SyntaxError)
}

pub fn from_pest_parsing(file: & std::path::PathBuf, err: & ParseError) -> CompileError {

    match err {
        ParseError::IOError(_io) => {
            CompileError::UnknwonFileIncludeError(UnknownFileIncludeError {
                file_error: file.clone(),
                source_error: "TODO".to_string(),
                span_error: (10, 12),

                invalid_file: file.clone()
            })
        },
        ParseError::PestError(_pest) => {
            CompileError::SyntaxError(syntax::SyntaxError {
                file_error: file.clone(),
                source_error: "TODO".to_string(),
                span_error: (10, 12)
            })
        }
    }

}


