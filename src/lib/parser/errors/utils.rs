fn get_line_num(idx: usize, data: & std::string::String) -> usize {
    data.as_str()[0..idx].matches("\n").count()
}

fn get_line(line: usize, data: & std::string::String) -> std::string::String {
    let lines: Vec<&str> = data.as_str().split('\n').collect();
    lines.as_slice()[line].to_string()
}

pub fn get_content_at_span(data: & std::string::String, span: (usize, usize)) -> std::string::String {
    let lines: (usize, usize) = (
        get_line_num(span.0, data),
        get_line_num(span.1, data)
    );

    let to_collect: Vec<usize> = ((lines.0)..(lines.1 + 1)).collect();
    let collected: Vec<std::string::String> = to_collect.into_iter()
        .map(|line_num| format!("{} | {}", line_num + 1, get_line(line_num, data)))
        .collect();

    collected.join("\n")

}