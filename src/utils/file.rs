pub fn replace_string_in_file(file_path: &str, old_str: &str, new_str: &str) {
    let file_content = std::fs::read_to_string(file_path).expect("Failed to read file");

    let replaced_content = file_content.replace(old_str, new_str);

    std::fs::write(file_path, replaced_content).expect("Failed to write file");
}
