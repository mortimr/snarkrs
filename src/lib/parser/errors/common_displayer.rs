use std::fmt;

pub fn common_displayer(formatter: &mut fmt::Formatter, name: & str, code: & usize, source: & std::string::String, _span: & (usize, usize), file: & std::path::PathBuf) -> fmt::Result {
    write!(formatter, "{}[E{}]:\nin {:?}\n\n\t{}\n\n", name, code, file, source)
}
