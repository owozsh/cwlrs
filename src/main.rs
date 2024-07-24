use std::fs;

use toml::Table;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = {
        let config_path = &args[1];
        let config_content = fs::read_to_string(&args[1]);
    };

    let output_dir_path = &args[2];
}
