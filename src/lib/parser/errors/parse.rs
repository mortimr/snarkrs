use pest;
use parser::ast;
use parser::errors;
use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    span: (usize, usize),
    buff: std::string::String
}

fn extract_line_containing_idx(source: & std::string::String, idx: usize) -> std::string::String {
    let mut start_idx = idx;
    let mut end_idx = idx;

    let source = source.as_str();


    while start_idx > 0 && source.chars().nth(start_idx) != Some('\n') {
        start_idx -= 1;
    }

    if source.chars().nth(start_idx) == Some('\n') {
        start_idx += 1;
    }

    while end_idx < source.len() && source.chars().nth(end_idx) != Some('\n') {
        end_idx += 1;
    }

    let line = &source[start_idx..(end_idx)];

    line.to_string()
}

fn gen_preline(line_num_size: usize) -> std::string::String {

    let len = line_num_size + 2;

    let mut bytes = vec![b' '; len];

    bytes[line_num_size + 1] = b'|';

    let uline = String::from_utf8(bytes);

    match uline {
        Ok(uline) => {
            uline
        },
        _ => "".to_string()
    }

}

fn gen_underline(len: usize, pos: usize, line_num_size: usize) -> std::string::String {

    let mut len = len + (line_num_size + 3);
    if len <= pos {
        len = pos + 1;
    }

    let mut bytes = vec![b' '; len];

    bytes[pos + line_num_size + 3] = b'^';
    bytes[line_num_size + 1] = b'|';

    let uline = String::from_utf8(bytes);

    match uline {
        Ok(uline) => {
            uline
        },
        _ => "".to_string()
    }

}

fn add_line_num(line: std::string::String, linenum: usize) -> std::string::String {
    format!("{} | {}", linenum, line)
}

fn get_buff_from_perror(source: & std::string::String, lcol: pest::error::LineColLocation, location: pest::error::InputLocation) -> std::string::String {
    match (lcol, location) {
        (pest::error::LineColLocation::Pos(lcol), pest::error::InputLocation::Pos(pos_1d)) => {
            let num_len = format!("{}", lcol.0).len();

             let raw_line = extract_line_containing_idx(source, pos_1d);
            let line = add_line_num(raw_line, lcol.0);

            let preline = gen_preline(num_len);
            let uline = gen_underline(line.len(), lcol.1 - 1, num_len);

            format!("{}\n{}\n{}", preline, line, uline)
        },
        (pest::error::LineColLocation::Span(_lcol_begin, _lcol_end), pest::error::InputLocation::Span(_pos_2d)) => {
            "MUTLI LINE ERROR IMPL PLEASE".to_string()
        }
        _ => {
            "".to_string()
        }
    }
}

impl ParseError {

    pub fn build(msg: std::string::String, code: usize, perror: pest::error::Error<ast::Rule>, source: & std::string::String) -> errors::CompileError {
        errors::CompileError {
            code,
            msg,
            infos: errors::ErrorInfos::ParseError(ParseError {
                span: (10, 10),
                buff: get_buff_from_perror(source, perror.line_col, perror.location)
            })
        }
    }

}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {

        write!(formatter, "{}", self.buff)

    }
}