//use parser::ast;
//use rental::RentalError;
//
//#[derive(Debug)]
//pub enum ContextError {
//    IOError(std::io::Error),
//    ParseError(ast::ParseError)
//}
//
//#[derive(Debug)]
//enum LoadAttempt {
//    Loaded(ast::rt::File),
//    Error(ContextError)
//}
//
//#[derive(Debug)]
//pub struct Context {
//    files: std::collections::HashMap<std::path::PathBuf, LoadAttempt>,
//    errors: Vec<(std::path::PathBuf)>,
//    main: std::path::PathBuf
//}
//
//fn context_gather_includes(file: & mut ast::rt::File) {
//
//    //let pairs_copy: pest::iterators::Pairs<ast::Rule> = file.rent(| ast | { ast.clone() });
//
//    fn climb_down(pairs: & pest::iterators::Pair<ast::Rule>) {
//
//    }
//
//}
//
//fn context_load_file_success(file: ast::rt::File,ctx: & mut Context, file_path: & std::path::PathBuf) {
//
//    ctx.files.insert(
//        file_path.clone(),
//        LoadAttempt::Loaded(file)
//    );
//
//    match ctx.files.get_mut(&file_path.clone()) {
//
//        Some(val) => {
//            match val {
//                LoadAttempt::Loaded(ref mut loaded_file) => {
//                    context_gather_includes(loaded_file);
//                },
//                LoadAttempt::Error(error) => panic!("Found Error inside HashMap, expected File")
//            }
//        },
//
//        None => panic!("Cannot access value from HashMap")
//
//    }
//
//}
//
//fn context_load_file_error(error: RentalError<ast::ParseError, std::boxed::Box<std::path::PathBuf>>, ctx: & mut //Context, file_path: & std::path::PathBuf) {
//
//    ctx.errors.push(((*error.1).clone()));
//    ctx.files.insert(
//        file_path.clone(),
//        LoadAttempt::Error(ContextError::ParseError(error.0))
//    );
//
//}
//
//fn context_load_file(ctx: & mut Context, file_path: & std::path::PathBuf) {
//
//    if !ctx.files.contains_key(file_path) {
//
//        match ast::parse_file(file_path) {
//
//            Ok(file) => context_load_file_success(file, ctx, file_path),
//
//            Err(error) => context_load_file_error(error, ctx, file_path)
//
//        };
//
//    }
//
//}
//
//pub fn build_context(file: & std::path::PathBuf) -> Result<Context, ContextError> {
//
//    let mut ctx = Context {
//        files: std::collections::HashMap::new(),
//        errors: Vec::new(),
//        main: file.clone()
//    };
//
//
//    context_load_file(& mut ctx, file);
//
//    Ok(ctx)
//}
//
//#[cfg(test)]
//mod compile_test {
//
//    use parser::compile;
//
//    #[test]
//    fn test_build_context() {
//
//        let path = std::fs::canonicalize("./src/lib/parser/test_material/circuits/bitify.circom").expect("IO //ERROR");
//
//        let ctx = match compile::build_context(&path) {
//            Ok(ctx) => ctx,
//            Err(error) => {
//                println!("IO Error");
//                panic!(error);
//            }
//        };
//
//        println!("ctx {:?}", ctx);
//    }
//}
