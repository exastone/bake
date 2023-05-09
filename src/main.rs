mod init_project;
mod rule;

use std::fs;
use std::io::Write;

use init_project::ProjectBuilder;
use rule::{create_rule, write_rule};

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
fn get_project_files(project_path: String) -> Vec<File> {
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
fn get_dependencies(project_path: String, build_files: &mut Vec<File>) {
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

fn create_makefile(
    project: ProjectBuilder,
    project_path: String,
    build_files: &Vec<File>,
) -> std::io::Result<()> {
    let mut makefile = fs::File::create(format!("{}/DRAFT-Makefile", project_path))?;

    let build_command = match project.build_command {
        Some(command) => command,
        None => String::from(""),
    };

    makefile.write_all(format!("{}.app : ", project.name).as_bytes())?;
    let main = build_files
        .iter()
        .find(|file| file.name == "main.c" || file.name == "main.cpp")
        .unwrap();

    makefile.write_all(format!("main.o").as_bytes())?;

    for dependency in &main.dependencies {
        makefile.write_all(
            format!(" {}.o", dependency.split(".").collect::<Vec<&str>>()[0]).as_bytes(),
        )?;
    }

    makefile.write(format!("\n\t{} -o {}.app ", build_command, project.name).as_bytes())?;

    for files in build_files {
        if matches!(files.file_kind, FileType::Source) {
            makefile.write_all(
                format!("{}.o ", files.name.split('.').collect::<Vec<&str>>()[0]).as_bytes(),
            )?;
        }
    }
    makefile.write(b"\n\n")?;

    // for all source files in write a Make rule with format:
    // <target_source_file>.o : <target_source_file> <dependencies>
    // \t<build_command> -c <target_source_file>
    for file in build_files {
        match file.file_kind {
            FileType::Source => {
                makefile.write_all(
                    format!(
                        "{}.o : {}",
                        file.name.split('.').collect::<Vec<&str>>()[0],
                        file.name
                    )
                    .as_bytes(),
                )?;
                for dependency in &file.dependencies {
                    makefile.write_all(format!(" {}", dependency,).as_bytes())?;
                }
                // makefile.write(b"\n\tcc -c ___\n")?;
                makefile.write(format!("\n\t{} -c {} \n", build_command, file.name).as_bytes())?;
            }
            FileType::Header => {}
        }
    }

    makefile.write_all(format!("clean :\n\trm {}.app *.o\n", project.name).as_bytes())?;

    Ok(())
}

fn main() {
    // let project: create_new_project::ProjectBuilder = create_new_project::initialize_project();
    // println!("{:?}", proj);

    // let project_path = "../C++ Test Project";
    // let mut build_files = get_project_files(project_path.to_string());
    // get_dependencies(project_path.to_string(), &mut build_files);

    // create_makefile(project, project_path.to_string(), &build_files).ok();
    let prereq: Vec<String> = vec![String::from("hello.c")];
    let recipe: Vec<String> = vec![String::from("clang -std=c17 -c hello.c")];
    let mut rule1 = create_rule(String::from("hello.o"), prereq, recipe);
    // create file and pass to write_rule for writing

    // let mut file = fs::File::create("MakeFoo").expect("error creating file");
    let mut file = fs::File::create("Makefoo").unwrap();

    write_rule(&mut file, &rule1);

    // add string to rule1.prereq
    rule1.prereqs.push(String::from("other.c"));
    write_rule(&mut file, &rule1);
}
