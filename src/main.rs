// mod init_project;
// mod rule;

// use std::fs;
// use std::io::Write;

// use init_project::ProjectBuilder;
// use rule::{create_rule, write_rule};
/*
enum FileType {
    Header,
    Source,
}

struct File {
    name: String,
    dependencies: Vec<String>,
    file_kind: FileType,
}
 */

mod bakefile;
mod clap_cli;
mod helpers;
use clap_cli::{clap_cli, Cli};

fn main() {
    let cli = <Cli as clap::Parser>::parse();
    clap_cli(cli)

    // Continued program logic goes here...
}

/* // let project: create_new_project::ProjectBuilder = create_new_project::initialize_project();
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
write_rule(&mut file, &rule1); */
