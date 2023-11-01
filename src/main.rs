use std::{env, fs::File};

use nebulang::engine::{
    compiler::Compiler,
    config::{extract_config, Config},
    parser::*,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config: Config;
    match extract_config(&args) {
        Ok(imported_config) => config = imported_config,
        Err(error) => panic!("{}", error),
    }

    println!("{:?}", config);

    let _current_dir_str = env::current_dir().unwrap().to_str().unwrap().to_owned();

    let file = File::open(config.src.clone());
    if let Ok(mut file) = file {
        let mut parser = Parser::new(config.clone());
        parser
            .parse(file)
            .expect(&format!("Could not finish parsing {}", config.src.clone()));
        Compiler::compile(&parser, &config);
    } else {
        panic!("Source file {} not found!", config.src);
    }
}
