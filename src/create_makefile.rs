pub fn create_makefile(
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
