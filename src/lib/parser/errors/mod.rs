pub mod parse;
pub mod io;
pub mod logic;
pub mod utils;

pub mod common_displayer;

use std::fmt;


#[derive(Debug)]
pub struct CompileError {
    pub msg: std::string::String,
    pub code: usize,
    pub infos: ErrorInfos
}

#[derive(Debug)]
pub enum ErrorInfos {
    ParseError(parse::ParseError),
    IOError(io::IOError),
    LogicError(logic::LogicError)
}

impl fmt::Display for CompileError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match &self.infos {
            ErrorInfos::ParseError(perror) => {
                write!(formatter, "E[{}]: ParseError\n\n{}\n\n\t{}\n", self.code, perror, self.msg)
            },
            ErrorInfos::IOError(ioerror) => {
                write!(formatter, "E[{}]: IOError\n\n{}\n\n\t{}\n", self.code, ioerror, self.msg)
            },
            ErrorInfos::LogicError(lerror) => {

                write!(formatter, "E[{}]: LogicError\n\n{}\n\n\t{}\n", self.code, lerror, self.msg)
            }
        }
    }
}

