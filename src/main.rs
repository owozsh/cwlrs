use std::{collections::HashMap, fs, io::Write};
use toml::Table;

fn read_template(template_file_name: &str) -> String {
    let template_path = format!("/home/owozsh/Developer/cwlrs/templates/{template_file_name}");
    fs::read_to_string(&template_path).expect("Error reading the template file.")
}

fn get_template_path(template_type: &str) -> Result<&str, &str> {
    match template_type {
        "vscode" => Ok("vscode.json"),
        _ => Err("Not a valid template."),
    }
}

fn create_color_map(config: &Table) -> HashMap<&str, &str> {
    let color_scopes = ["base", "colors", "syntax", "ui"];
    let mut colors: HashMap<&str, &str> = HashMap::new();

    for scope in color_scopes {
        let err = format!("Missing {scope} key");
        let scope_items = config[scope].as_table().expect(&err).iter();

        for (key, value) in scope_items {
            colors.insert(key, value.as_str().unwrap());
        }
    }

    colors
}

fn generate_from_template(template_type: &str, config: &Table) -> String {
    let template = read_template(get_template_path(template_type).unwrap());

    let mut result = String::from(&template);

    let color_map = create_color_map(config);

    for (key, value) in color_map.iter() {
        let key = &format!("${key}");

        result = match &value[..1] {
            "#" => result.replace(key, value),
            _ => result.replace(key, color_map.get(value).unwrap()),
        };
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

    write_output(&output_dir_path, &generate_from_template("vscode", &config));
}
