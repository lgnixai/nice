use std::fs;
use std::path::{Path, PathBuf};
use log::debug;
use crate::parser;
use crate::parser::config::Config;
use crate::parser::{Diagnostic, ParsingCtx, Root};
use crate::parser::source_file::SourceFile;


pub fn compile_file(in_name: String, config: &Config) -> Result<(), Diagnostic> {
    let mut source_file = SourceFile::from_file(in_name)?;

    if config.std {
        source_file.content = "mod std\nuse std::prelude::(*)\n".to_owned() + &source_file.content;
    }

    source_file.mod_path = PathBuf::from("root");
    println!("1");
    compile_str(&source_file, config)
}

pub fn compile_str(input: &SourceFile, config: &Config) -> Result<(), Diagnostic> {
    let mut parsing_ctx = ParsingCtx::new(config);
    //println!("2:parsing_ctx {:?}",parsing_ctx.get_current_file());
    parsing_ctx.add_file(input);
    println!("2:parsing_ctx {:?}",parsing_ctx.get_current_file());

    println!("3");

    let hir = parse_str(&mut parsing_ctx, config)?;
    println!("4");

    println!("{:#?}",hir);
    //generate_ir(hir, config)?;

    //parsing_ctx.print_success_diagnostics();

    Ok(())
}

pub fn parse_str(parsing_ctx: &mut ParsingCtx, config: &Config) -> Result<Root, Diagnostic> {
    // Text to Ast
    debug!("    -> Parsing");

    println!("5");
    let mut ast = parser::parse(parsing_ctx)?;
    println!("6{:#?}",ast);

    // Name resolving
    debug!("    -> Resolving");
    //resolver::resolve(&mut ast, parsing_ctx)?;
    //
    //  // Lowering to HIR
    //  debug!("    -> Lowering to HIR");
    // // let mut hir = ast_lowering::lower_crate(&ast);
    //
    //  // Infer Hir
    //  debug!("    -> Infer HIR");
    //  //let new_hir = infer::infer(&mut hir, parsing_ctx, config)?;

    Ok(ast)
}

// pub fn generate_ir(hir: hir::Root, config: &Config) -> Result<(), Diagnostic> {
//     // Generate code
//     debug!("    -> Lower to LLVM IR");
//     codegen::generate(config, hir)?;
//
//     Ok(())
// }

#[test]
fn main() {
    let entry_file = "src/engine/script/1.ps";


    let path = Path::new("src/lib/").join("/react");

    println!("{:?}",path);
    let build_path = path.parent().unwrap().join("/react/build");

    println!("{:?}",build_path);

    let mut config = Config::default();

    config.project_config.entry_point = PathBuf::from(path);
    config.quiet = true;

    config.build_folder = build_path;

    // println!("{:?}",config.build_folder.clone());
    //
    // fs::create_dir_all(config.build_folder.clone()).unwrap();


    if let Err(diagnostic) = compile_file(entry_file.to_string(), &config) {

            println!("Error: {}", diagnostic.get_kind());



    }



}