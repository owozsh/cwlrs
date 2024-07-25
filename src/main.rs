use std::{fs, io::Write};
use toml::Table;

fn generate_from_template(config: Table) -> String {
    let template =
        fs::read_to_string("./templates/vscode.json").expect("Error reading the template file.");

    let mut result = String::new();

    for (key, value) in config["base"].as_table().expect("Missing base key").iter() {
        let from = format!("${key}");
        dbg!(&from);
        let to = value.as_str().expect(&format!("Missing {key} key."));
        dbg!(&to);
        result = template.replace(&from, &to);
    }

    result
}

fn write_output(output_dir_path: &str, content: &str) {
    let mut output = fs::File::create(output_dir_path).expect("Error creating output");

    output
        .write_all(content.as_bytes())
        .expect("Error writing output");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = {
        let config_path = &args[1];
        let config_content = fs::read_to_string(config_path).expect("Not a valid config file.");
        toml::from_str(&config_content).expect("Not a valid toml file")
    };

    let output_dir_path = &args[2];

    write_output(&output_dir_path, &generate_from_template(config));
}
