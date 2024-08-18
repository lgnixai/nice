use std::{fs, io};
use std::path::{Path, PathBuf};

use parse::config::Config;
use parse::eval_ast::compile_file;

#[test]
fn main() -> io::Result<()> {
    //let input = fs::read_to_string("main.tsx")?;
    let entry_file = "script/1.ps";


    let path = Path::new("src/lib/").join("/react");

    println!("{:?}",path);
    let build_path = path.parent().unwrap().join("/react/build");

    println!("{:?}",build_path);

    let mut config =  Config::default();

    config.project_config.entry_point = PathBuf::from(path);
    config.quiet = true;

    config.build_folder = build_path;

    // println!("{:?}",config.build_folder.clone());
    //
    // fs::create_dir_all(config.build_folder.clone()).unwrap();


    if let Err(diagnostic) = compile_file(entry_file.to_string(), &config) {

        println!("Error: {}", diagnostic.get_kind());



    }
    // println!(
    //     "[\n{}\n]",
    //     ast.value
    //         .iter()
    //         .map(|token| format!("  {token:#?}"))
    //         .collect::<Vec<_>>()
    //         .join(",\n")
    // );

    Ok(())
}
