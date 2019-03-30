use parser::errors;
use std::fmt;
use parser::errors::utils;

#[derive(Debug)]
pub struct LogicError {
    file: std::path::PathBuf,
    buff: Option<std::string::String>
}

impl LogicError {

    pub fn build(msg: std::string::String, code: usize, file: std::path::PathBuf) -> errors::CompileError {
        errors::CompileError {
            code,
            msg,
            infos: errors::ErrorInfos::LogicError(LogicError {
                file,
                buff: None
            })
        }
    }

    pub fn build_with_span(msg: std::string::String, code: usize, file: std::path::PathBuf, data: &std::string::String, span: (usize, usize)) -> errors::CompileError {
        errors::CompileError {
            code,
            msg,
            infos: errors::ErrorInfos::LogicError(LogicError {
                file,
                buff: Some(utils::get_content_at_span(data, span))
            })
        }
    }


}

impl fmt::Display for LogicError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {

        match &self.buff {
            Some(data) => {
                write!(formatter, "In {:#?}:\n\n{}", self.file, data)
            },
            None => {
                write!(formatter, "In {:#?}:", self.file)
            }
        }

    }
}