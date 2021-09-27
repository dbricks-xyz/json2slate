use std::fs;
use std::path::PathBuf;

use serde_json::Value;

fn main() {
    fs::remove_dir_all("out").unwrap();
    fs::create_dir("out").unwrap();
    in_to_out("./in");
    println!("All done.")
}

fn in_to_out(starting_path: &str) {
    for entry in fs::read_dir(starting_path).unwrap() {
        let in_path = entry.unwrap().path();
        if in_path.is_dir() {
            let out_path = in_path.to_str().unwrap().replace("in/", "out/");
            let _ = fs::create_dir(out_path);
            in_to_out(in_path.to_str().unwrap());
        } else {
            let data = generate_md_file(&in_path);
            let out_path = in_path
                .to_str()
                .unwrap()
                .replace("in/", "out/")
                .replace(".json", ".md.erb");
            fs::write(out_path, data).unwrap();
        }
    }
}

fn generate_md_file(in_path: &PathBuf) -> String {
    let mut output = String::new();
    let original = parse_json_file(in_path.clone());
    // --------------------------------------- request
    output.push_str("> Request\n\n");
    // route
    output.push_str("```\n");
    output.push_str(&format!("{}\n", &original["route"].to_string().clean()));
    output.push_str("```\n\n");
    // json
    output.push_str("```json\n");
    output.push_str(&format!(
        "{}\n",
        serde_json::to_string_pretty(&original["request"]).unwrap()
    ));
    output.push_str("```\n\n");
    // shell
    output.push_str("```shell\n");
    output.push_str(&format!(
        "{}\n",
        serde_json::to_string_pretty(&original["requestSchema"]).unwrap()
    ));
    output.push_str("```\n\n");
    // --------------------------------------- response
    output.push_str("> Response\n\n");
    // json
    output.push_str("```json\n");
    output.push_str(&format!(
        "{}\n",
        serde_json::to_string_pretty(&original["response"]).unwrap()
    ));
    output.push_str("```\n\n");
    // shell
    output.push_str("```shell\n");
    output.push_str(&format!(
        "{}\n",
        serde_json::to_string_pretty(&original["responseSchema"]).unwrap()
    ));
    output.push_str("```\n");

    output
}

fn parse_json_file(path: PathBuf) -> Value {
    let data = fs::read_to_string(path).expect("failed to read file");
    serde_json::from_str(&data).unwrap()
}

trait Clean {
    fn clean(&mut self) -> Self;
}

impl Clean for String {
    fn clean(&mut self) -> Self {
        String::from(self.trim_start_matches("\"").trim_end_matches("\""))
    }
}
