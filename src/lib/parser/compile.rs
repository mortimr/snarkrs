use parser::ast;
use parser::matchers;
use parser::errors;

#[derive(Debug)]
enum LoadAttempt {
    Loading(),
    Loaded(ast::File),
    Error()
}

#[derive(Debug)]
pub struct Context {
    files: std::collections::HashMap<std::path::PathBuf, LoadAttempt>,
    errors: Vec<errors::CompileError>,
    main: std::path::PathBuf,
    include_stack: Vec<std::path::PathBuf>
}

fn context_gather_includes(file: & mut ast::File) {

    {
        if file.root.ast.len() != 1 {
            panic!("Invalid ast");
        }
    }

    let mut includes: Vec<std::path::PathBuf> = Vec::new();
    let current_file_dir: std::path::PathBuf = match file.path.parent() {
        Some(parent) => parent,
        None => panic!("Cannot retrieve parent directory of source file !")
    }.to_path_buf();

    if let ast::tokens::Token::NonTerminal(circuit) = &file.root.ast[0] {
        for token in &circuit.subrules {
            match token {
                ast::tokens::Token::NonTerminal(nt) => {
                    if nt.rule == ast::Rule::IncludeStatement {
                        let file_name = matchers::include_statement::process_include_statement(nt);
                        let mut include_absolute_path = current_file_dir.clone();
                        include_absolute_path.push(file_name);
                        includes.push(include_absolute_path);
                    }
                },
                _ => {}
            }
        }
    } else {
        panic!("Should find Circuit at root");
    }

    file.includes = includes;


}

fn context_load_file_success(file: & mut ast::File) {

    context_gather_includes(file);

}

fn add_error_to_context(error: errors::CompileError, ctx: & mut Context) {
    ctx.errors.push(error);
}

fn context_load_file_error(error: ast::ParseError, ctx: & mut Context, file_path: & std::path::PathBuf) {

    add_error_to_context(errors::from_pest_parsing(&file_path.clone(), &error), ctx);

}

fn context_load_file(ctx: & mut Context, file_path: & std::path::PathBuf) -> Option<ast::File> {

    if ctx.include_stack.contains(file_path) {
        return None;
    }

    let res = match ast::parse_file(file_path) {

        Ok(mut file) => {
            context_load_file_success(& mut file);
            file
        },

        Err(error) => {
            context_load_file_error(error, ctx, file_path);
            return None;
        }

    };

    ctx.include_stack.push(file_path.clone());

    for include in &res.includes {
        match context_load_file(ctx, include) {
            Some(file) => {
                let path = file.path.clone();

                ctx.files.insert(path, LoadAttempt::Loaded(file));
            },
            None => {}
        }
    }


    ctx.include_stack.pop();

    Some(res)
}

///
/// Takes a path as input, recovers all ASTs of given main files and subsequent includes
///
/// Does not check for anything else. It just builds all the AST, set the `include` values on the `File`
/// structure and reports possible errors.
///
pub fn build_context(file: & std::path::PathBuf) -> Context {

    let mut ctx = Context {
        files: std::collections::HashMap::new(),
        errors: Vec::new(),
        main: file.clone(),
        include_stack: Vec::new()
    };

    ctx.files.insert(file.clone(), LoadAttempt::Loading());

    let main: LoadAttempt = match context_load_file(& mut ctx, file) {
        Some(file) => LoadAttempt::Loaded(file),
        None => LoadAttempt::Error()
    };

    ctx.files.insert(file.clone(), main);

    ctx

}

#[cfg(test)]
mod compile_test {

    use parser::compile;

    use galvanic_assert::matchers::*;

    #[test]
    fn test_build_context_from_invalid_file() {

        let path = std::fs::canonicalize("./Cargo.toml").expect("Invalid Path");

        let ctx = compile::build_context(&path);

        expect_that!(&ctx.errors.len(), is(eq(1)));
        expect_that!(&ctx.files.keys().len(), is(eq(1)));


    }

    #[test]
    fn test_build_context() {

        let path = std::fs::canonicalize("./src/lib/parser/test_material/circuits/bitify.circom").expect("Invalid Path");

        let ctx = compile::build_context(&path);

        expect_that!(&ctx.errors.len(), is(eq(0)));
        expect_that!(&ctx.files.keys().len(), is(eq(5)));


    }
}
