enum FileType {
    Header,
    Source,
}

struct File {
    name: String,
    dependencies: Vec<String>,
    file_kind: FileType,
}

// get list of all files in project directory
pub fn get_project_files(project_path: String) -> Vec<File> {
    let mut build_files: Vec<File> = Vec::new();

    let paths = fs::read_dir(project_path).unwrap();
    for path in paths {
        let file_kind: FileType;
        let file_name = String::from(path.unwrap().path().file_name().unwrap().to_str().unwrap());

        if file_name.ends_with(".h") {
            file_kind = FileType::Header;
        } else if file_name.ends_with(".c") {
            file_kind = FileType::Source;
        } else if file_name.ends_with(".cpp") {
            file_kind = FileType::Source;
        } else {
            break;
        }

        build_files.push(File {
            name: file_name,
            dependencies: Vec::new(),
            file_kind,
        });
    }

    return build_files;
}

// get directory dependencies
pub fn get_dependencies(project_path: String, build_files: &mut Vec<File>) {
    for file in build_files.iter_mut() {
        let contents = fs::read_to_string(format!("{}/{}", project_path, file.name)).unwrap();
        for line in contents.lines() {
            if line.starts_with("#include\"") || line.starts_with("#include \"") {
                let line: String = line.split(" ").collect::<Vec<&str>>()[1].to_string();
                let line = line.replace(['"', '<', '>'], "");
                file.dependencies.push(line);
            }
        }
    }
}
