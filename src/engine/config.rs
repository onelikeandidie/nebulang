// use std::fs;
// use std::env;
// use std::path::Path;

use super::util::get_index;

#[derive(Debug)]
pub struct ConfigFile {
    pub src: String,
    pub out: String,
    pub low_mem: bool,
    pub string_delimiter: char,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub src: String,
    pub out: String,
    pub origin: String,
    pub debbuging: bool,
    pub low_mem: bool,
    pub string_delimiter: char,
}

pub fn extract_config(args: &Vec<String>) -> Result<Config, String> {
    // // Check if the --config flag was passed
    // // And find the file indicated
    // if args.contains(&"--config".to_string()) {
    //     // Get the index of the --config flag in the arguments
    //     let index_of_flag = get_index(&args, "--config");
    //     if index_of_flag < 0 {
    //         return Err("Config given, attempting to load".to_string());
    //     }
    //     // Check if there is a path after the config flag
    //     if let Some(path) = args.get((index_of_flag + 1) as usize) {
    //         return load_config(path);
    //     } else {
    //         println!("Config path not passed after flag, please use `--config \"path/to/config\"`");
    //     }
    // }

    // Check if instead, the arguments were passed
    let has_src         = args.contains(&"--src".to_string());
    let has_out         = args.contains(&"--out".to_string());
    let has_low_mem     = args.contains(&"--low-mem".to_string());
    let has_str_del     = args.contains(&"--qq".to_string());
    if has_src && has_out {
        let index_of_src        = (get_index(&args, "--src") + 1) as usize;
        let index_of_target     = (get_index(&args, "--out") + 1) as usize;
        let index_of_str_delim  = (get_index(&args, "--qq" ) + 1) as usize;
        let src = args.get(index_of_src).unwrap_or(&"./src".to_string()).to_owned();
        let out = args.get(index_of_target).unwrap_or(&"./target".to_string()).to_owned();
        let mut del = if has_str_del {
            args.get(index_of_str_delim).unwrap_or(&"\"".to_string()).to_owned()
        } else {
            "\"".to_string()
        };
        return Ok(Config {
            src,
            out,
            origin: ".".to_string(),
            debbuging: false,
            low_mem: has_low_mem,
            string_delimiter: del.pop().expect("--qq passed but no argument"),
        });
    }

    // // Maybe the config is in the root where this was called
    // let current_dir = env::current_dir().unwrap();
    // let current_dir_string = current_dir.to_str().unwrap().to_owned();
    // let possible_config_path_string = current_dir_string + "/php-injector.json";
    // let possible_config_path = Path::new(&possible_config_path_string);
    // let config_exists = possible_config_path.exists();
    // if config_exists {
    //     return load_config(&possible_config_path_string);
    // }

    return Err("No configuration provided!".to_string());
}

// fn load_config(path: &str) -> Result<Config, String> {
//     let contents = fs::read_to_string(path);
//     let json;
//     match contents {
//         Ok(txt) => {
//             json = serde_json::from_str::<ConfigFile>(&txt);
//         },
//         Err(_) => {
//             let error_msg = format!("Could not retrieve file contents! {}", path);
//             return Err(error_msg);
//         },
//     }
//     match json {
//         Ok(config) => {
//             // Attach origin config parent dir path
//             let path_obj = Path::new(path).parent().unwrap();
//             return Ok(Config {
//                 injections: config.injections,
//                 src: config.src,
//                 cache: config.cache,
//                 origin: path_obj.to_str().unwrap().to_string(),
//                 use_document_root: config.use_document_root.unwrap_or(true),
//                 copy_other: config.use_document_root.unwrap_or(false),
//                 debbuging: false
//             });
//         },
//         Err(_) => return Err("Could not parse config file!".to_string()),
//     }
// }