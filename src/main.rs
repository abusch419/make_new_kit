use std::fs;
use regex::Regex;
use std::path::PathBuf;
use rayon::prelude::*;
use std::io::Write;

mod xml_boilerplate;

fn main() {
    println!("Hello, World! 🌍");
    if let Err(e) = build_xml_file_from_filenames(
        "/Users/alexandereversbusch/Desktop/Downloaded Sample Packs/Arthur Duboise/".to_string()
        // "/Users/alexandereversbusch/Desktop/Downloaded Sample Packs/RSKT/".to_string()
    ) {
        println!("Error: {}", e);
    }
}

fn build_xml_file_from_filenames(filepath: String) -> Result<(), Box<dyn std::error::Error>> {
    let filenames = get_file_names_from_file_path(&filepath)?;
    let mut xml_strings = Vec::new();

    for filename in &filenames {
        xml_strings.push(generate_xml_for_filename(filename));
    }

    for (index, chunk) in xml_strings.chunks(8).enumerate() {
        let combined_xml = chunk.join("\n");
        save_to_xml_file(&combined_xml, "desired_folder_name", &format!("desired_filename_{}", index))?;
    }

    Ok(())
}

fn generate_xml_for_filename(filename: &str) -> String {
    let name = filename.split('/').last().unwrap_or(filename);
    format!("{} {} {}", xml_boilerplate::SOUND_TAG_BOILERPLATE, name, filename)
}

fn save_to_xml_file(data: &str, folder_name: &str, filename: &str) -> std::io::Result<()> {
    let folder_path = format!("/Users/alexandereversbusch/Desktop/{}", folder_name);
    fs::create_dir_all(&folder_path)?;

    let file_path = format!("{}/{}.xml", folder_path, filename);
    let mut file = fs::File::create(file_path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}


fn get_file_names_from_file_path(filepath: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let path = PathBuf::from(filepath);
    if !path.exists() || !path.is_dir() {
        return Err(format!("Path '{}' either doesn't exist or isn't a directory", filepath).into());
    }

    let mut filenames: Vec<String> = Vec::new();

    let entries: Vec<_> = fs::read_dir(&path)?.collect();
    filenames.par_extend(entries.par_iter().filter_map(|entry| {
        let entry = entry.as_ref().ok()?;
        if entry.path().is_dir() {
            let new_path = path.join(entry.file_name());
            get_file_names_from_file_path(new_path.to_str()?).ok()
        } else {
            if !entry.file_name().to_string_lossy().starts_with('.') {
                Some(vec![entry.file_name().to_string_lossy().to_string()])
            } else {
                None
            }
        }
    }).flatten());

    let re_start = Regex::new(r"^(\d+)")?;
    let re_end = Regex::new(r"(\d+)\.wav$")?;
    let re_parenthesis = Regex::new(r"\((\d+)\)")?; 

    filenames.par_sort_by(|a, b| {
        let type_a = extract_type(a);
        let type_b = extract_type(b);
        type_a.cmp(&type_b).then_with(|| {
            let num_a = extract_number(a, &re_start, &re_end, &re_parenthesis);
            let num_b = extract_number(b, &re_start, &re_end, &re_parenthesis);
            num_a.cmp(&num_b)
        })
    });

    Ok(filenames)
}

// ======= Sorting Logic =======
fn extract_type(filename: &str) -> String {
    let re = Regex::new(r"(\d+\s*(?i)bpm|\D+)").unwrap();
    let mut type_string = String::new();
    for cap in re.captures_iter(filename) {
        type_string.push_str(&cap[0]);
    }
    type_string
}

fn extract_number(filename: &str, re_start: &Regex, re_end: &Regex, re_parenthesis: &Regex) -> i32 {
    if let Some(cap) = re_start.captures(filename) {
        cap[1].parse().unwrap_or(0)
    } else if let Some(cap) = re_end.captures(filename) {
        cap[1].parse().unwrap_or(0)
    } else if let Some(cap) = re_parenthesis.captures(filename) {
        cap[1].parse().unwrap_or(0)
    } else {
        0
    }
}
