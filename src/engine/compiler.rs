use std::fs::File;
use std::io::prelude::*;

use super::{parser::Parser, config::Config};

pub struct Compiler;

impl Compiler {
    pub fn compile(parser: &Parser, config: &Config) {
        let file = File::create(config.out.clone());
        match file {
            Ok(mut file) => {
                    // println!("{}", token);
                file.write(format!("{:#?}\n", parser.symbols).as_bytes())
                    .expect(
                        format!("Could not write to file: {}", config.out.clone())
                        .as_str()
                    );

                file.write("\n".as_bytes())
                    .expect(
                        format!("Could not write to file: {}", config.out.clone())
                        .as_str()
                    );
                
                file.write(format!("{:#?}\n", parser.nodes).as_bytes())
                    .expect(
                        format!("Could not write to file: {}", config.out.clone())
                        .as_str()
                    );
            },
            Err(error) => {
                println!("Unable to open file {} {}", config.out.clone(), error);
            },
        }
    }
}