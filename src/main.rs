use std::fs;
use regex::Regex;
// use std::cmp::Ordering;


fn main() {
    println!("Hello, World! 🌍");
    build_xml_with_one_filename_and_save_it_on_my_desktop(
        // "/Users/alexandereversbusch/Desktop/Downloaded Sample Packs/RSKT/".to_string()
        "/Users/alexandereversbusch/Desktop/Downloaded Sample Packs/Arthur Duboise/loops/".to_string()
    );
}

fn build_xml_with_one_filename_and_save_it_on_my_desktop(filepath: String) -> () {
    let filenames = get_file_names_from_file_path(filepath);
    for filename in &filenames {
        println!("filename {:?}", filename);
    }
}

fn get_file_names_from_file_path(filepath: String) -> Vec<String> {
    let path = std::path::Path::new(&filepath);
    let mut filenames: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.path().is_dir() {
                    let mut new_path = filepath.clone();
                    new_path.push_str(entry.file_name().to_str().unwrap());
                    new_path.push_str("/");

                    let mut new_filenames = get_file_names_from_file_path(new_path);
                    filenames.append(&mut new_filenames);
                } else {
                    if !entry.file_name().to_string_lossy().starts_with('.') {
                        filenames.push(entry.file_name().to_string_lossy().to_string());
                    }
                }
            }
        }
    } else {
        println!("Error reading directory");
    }

    filenames.sort_by(|a, b| {
        let type_a = extract_type(a);
        let type_b = extract_type(b);
        type_a.cmp(&type_b).then_with(|| {
            let num_a = extract_number(a);
            let num_b = extract_number(b);
            num_a.cmp(&num_b)
        })
    });

    filenames
}


fn extract_type(filename: &str) -> String {
    let re = Regex::new(r"(\d+\s*(?i)bpm|\D+)").unwrap();
    let mut type_string = String::new();
    for cap in re.captures_iter(filename) {
        type_string.push_str(&cap[0]);
    }
    type_string
}

fn extract_number(filename: &str) -> i32 {
    let re_start = Regex::new(r"^(\d+)").unwrap();
    let re_end = Regex::new(r"(\d+)\.wav$").unwrap();
    if let Some(cap) = re_start.captures(filename) {
        cap[1].parse().unwrap_or(0)
    } else if let Some(cap) = re_end.captures(filename) {
        cap[1].parse().unwrap_or(0)
    } else {
        0
    }
}

