use parser::errors;
use std::fmt;

#[derive(Debug)]
pub struct IOError {
    pub file: std::path::PathBuf,
    pub error: std::io::Error
}

impl IOError {

    pub fn build(msg: std::string::String, code: usize, io: std::io::Error, file: & std::path::PathBuf) -> errors::CompileError {
        errors::CompileError {
            code,
            msg,
            infos: errors::ErrorInfos::IOError(IOError {
                file: file.clone(),
                error: io
            })
        }
    }

}

impl fmt::Display for IOError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {

        write!(formatter, "{:#?}: {}", self.file, self.error)

    }
}