///
/// E201: Unknown File Include Error
///
/// An include statement contained a value that raised an error while reading it.
///
#[derive(Debug, Clone)]
pub struct UnknownFileIncludeError {
    pub file_error: std::path::PathBuf,
    pub source_error: std::string::String,
    pub span_error: (usize, usize),

    pub invalid_file: std::path::PathBuf
}

use std::fmt;
use parser::errors::common_displayer;

impl fmt::Display for UnknownFileIncludeError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {

        common_displayer::common_displayer(formatter, "UnknownFileInclude", &201, &self.source_error, &self.span_error, &self.file_error)
            .and(write!(formatter, "\tUnknown File: {:?}\n", self.invalid_file))


    }
}

