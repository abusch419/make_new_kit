use std::fs;
use regex::Regex;
use std::path::PathBuf;
use rayon::prelude::*;
use std::io::Write;
use tera::Tera;

fn main() {
    println!("Hello, World! 🌍");
    if let Err(e) = build_xml_file_from_filenames(
        "/Users/alexandereversbusch/Desktop/rskt transfer test/".to_string()
    ) {
        println!("Error: {}", e);
    }
}

fn build_xml_file_from_filenames(filepath: String) -> Result<(), Box<dyn std::error::Error>> {
    let file_paths = get_file_paths_from_file_path(&filepath)?;

    let first_half_of_wrapper = include_str!("first_half_of_xml_wrapper_boilerplate.xml");
    let second_half_of_wrapper = include_str!("second_half_of_xml_wrapper_boilerplate.xml");

    for (index, chunk) in file_paths.chunks(8).enumerate() {
        let mut combined_xml = String::new();
        combined_xml.push_str(first_half_of_wrapper);

        for (filename, file_path) in chunk {
            let sound_xml = generate_xml_for_filename(&filename, file_path.to_str().unwrap());
            combined_xml.push_str("\n");
            combined_xml.push_str(&sound_xml);
            combined_xml.push_str("\n");
        }

        combined_xml.push_str(second_half_of_wrapper);

        save_to_xml_file(
            &combined_xml,
            "rskt_kicks",
            &format!("~ RSKT Kicks {}", index)
        )?;
    }

    Ok(())
}

fn generate_xml_for_filename(name: &str, filename: &str) -> String {
    let mut tera = Tera::default();
    tera.add_raw_template("xml_template", include_str!("sound_tag_boilerplate.xml"))
        .unwrap();

    let mut context = tera::Context::new();
    context.insert("name", name);
    // this should be called filepath, but I'm too lazy to change it right now
    let filename_modified_for_deluge = filename.replace("/Users/alexandereversbusch/Desktop/rskt transfer test/", "SAMPLES/RSKT Sample Pack/");
    context.insert("filename", &filename_modified_for_deluge);

    tera.render("xml_template", &context).unwrap()
}

fn save_to_xml_file(data: &str, folder_name: &str, filename: &str) -> std::io::Result<()> {
    let folder_path = format!("/Users/alexandereversbusch/Desktop/{}", folder_name);
    fs::create_dir_all(&folder_path)?;

    let file_path = format!("{}/{}.xml", folder_path, filename);
    let mut file = fs::File::create(file_path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

fn get_file_paths_from_file_path(
    filepath: &str
) -> Result<Vec<(String, PathBuf)>, Box<dyn std::error::Error>> {
    let path = PathBuf::from(filepath);
    if !path.exists() || !path.is_dir() {
        return Err(format!("Path '{}' either doesn't exist or isn't a directory", filepath).into());
    }

    let mut file_paths: Vec<(String, PathBuf)> = Vec::new();

    let entries: Vec<_> = fs::read_dir(&path)?.collect();
    for entry in entries {
        let entry = entry?;
        if entry.path().is_dir() {
            if let Some(new_path) = entry.path().to_str() {
                let sub_file_paths = get_file_paths_from_file_path(new_path)?;
                file_paths.extend(sub_file_paths);
            }
        } else {
            if !entry.file_name().to_string_lossy().starts_with('.') {
                file_paths.push((
                    entry.file_name().to_string_lossy().to_string(),
                    entry.path().to_path_buf(),
                ));
            }
        }
    }

    let re_start = Regex::new(r"^(\d+)")?;
    let re_end = Regex::new(r"(\d+)\.wav$")?;
    let re_parenthesis = Regex::new(r"\((\d+)\)")?;

    file_paths.par_sort_by(|(filename_a, _), (filename_b, _)| {
        let type_a = extract_type(filename_a);
        let type_b = extract_type(filename_b);
        type_a.cmp(&type_b).then_with(|| {
            let num_a = extract_number(filename_a, &re_start, &re_end, &re_parenthesis);
            let num_b = extract_number(filename_b, &re_start, &re_end, &re_parenthesis);
            num_a.cmp(&num_b)
        })
    });

    for file_path in &file_paths {
        println!("{} {}", file_path.0, file_path.1.display());
    }

    Ok(file_paths)
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

fn extract_number(filename: &str, re_start: &Regex, re_end: &Regex, re_parenthesis: &Regex) -> u32 {
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
