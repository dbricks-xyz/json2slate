use serde_json::Value;
use std::fs::File;
use std::io::BufRead;
use std::path::PathBuf;
use std::{fs, io};

fn main() {
    in_to_out("./in");
}

fn in_to_out(starting_path: &str) {
    fs::create_dir("out");
    for entry in fs::read_dir(starting_path).unwrap() {
        let in_path = entry.unwrap().path();
        if in_path.is_dir() {
            let out_path = in_path.to_str().unwrap().replace("in/", "out/");
            fs::create_dir(out_path);
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
    output.push_str(&format!("{}\n", clean_str(&original["route"].to_string())));
    output.push_str("```\n\n");
    // json
    output.push_str("```json\n");
    output.push_str(&format!(
        "{}\n",
        serde_json::to_string_pretty(&original["req"]).unwrap()
    ));
    output.push_str("```\n\n");
    // shell
    output.push_str("```shell\n");
    output.push_str(&format!(
        "{}\n",
        serde_json::to_string_pretty(&original["reqSchema"]).unwrap()
    ));
    output.push_str("```\n\n");
    // --------------------------------------- response
    output.push_str("> Response\n\n");
    // json
    output.push_str("```json\n");
    output.push_str(&format!(
        "{}\n",
        serde_json::to_string_pretty(&original["res"]).unwrap()
    ));
    output.push_str("```\n\n");
    // shell
    output.push_str("```shell\n");
    output.push_str(&format!(
        "{}\n",
        serde_json::to_string_pretty(&original["resSchema"]).unwrap()
    ));
    output.push_str("```\n");

    output
}

fn clean_str(s: &str) -> &str {
    s.trim_start_matches("\"").trim_end_matches("\"")
}

fn parse_json_file(path: PathBuf) -> Value {
    let data = fs::read_to_string(path).expect("failed to read file");
    serde_json::from_str(&data).unwrap()
}

// fn write_md_file(data: Value) {
//     let route = clean_route(&data["route"]);
//     println!("route is {}", route.to_string());
//     let req = &data["req"];
//     let res = &data["res"];
//
//     //can optimize futher with end_writing
//     let mut start_writing = false;
//     let mut relevant_lines = vec![];
//
//     let md_file = File::open("./out/index.html.md").unwrap();
//     let reader = io::BufReader::new(md_file);
//     for (i, line) in reader.lines().enumerate() {
//         let line = line.unwrap();
//         if start_writing {
//             relevant_lines.push(line);
//         } else if line.contains(&route.to_string()) {
//             println!("found line! {}", i);
//             start_writing = true;
//         }
//     }
//     //match indices once > save to temp > match indices 4 times > replace 1/2 and 3/4
//     //
// }
//
// fn clean_route(route: &Value) -> String {
//     let route_str = route.to_string();
//     String::from(
//         route_str
//             .trim_start_matches("\"")
//             .trim_end_matches("\"")
//             .trim_end_matches("/"),
//     )
// }

// trait Clean {
//     fn clean(&mut self) -> Self {
//         return self.trim_start_matches("\"");
//     }
// }
// impl Clean for String {}
