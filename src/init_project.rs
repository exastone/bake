// take user input for project name

use std::io;

#[derive(Debug)]
pub enum Language {
    C,
    CXX,
}

#[derive(Debug)]
pub enum Standard {
    C11,
    C17,
    CXX11,
    CXX14,
    CXX17,
    CXX2a,
}

#[derive(Debug)]
pub struct ProjectBuilder {
    pub name: String,
    pub language: Option<Language>,
    pub standard: Option<Standard>,
    pub build_command: Option<String>,
}

impl ProjectBuilder {
    pub fn new(name: &str) -> ProjectBuilder {
        ProjectBuilder {
            name: name.to_owned(),
            language: None,
            standard: None,
            build_command: None,
        }
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    pub fn standard(mut self, standard: Standard) -> Self {
        self.standard = Some(standard);
        self
    }

    pub fn build_command(mut self, build_command: &str) -> Self {
        self.build_command = Some(build_command.to_owned());
        self
    }
}

pub fn initialize_project() -> ProjectBuilder {
    let mut project: ProjectBuilder = ProjectBuilder::new("");

    let mut project_name: String = String::from("");
    let mut project_language: String = String::from("");
    let mut standard: String = String::from("");

    println!("Enter project name: ");
    io::stdin()
        .read_line(&mut project_name)
        .expect("Failed to read line");

    project.name = project_name.trim().to_string();

    println!("Choose project language:\n1. C\n2. C++");
    io::stdin()
        .read_line(&mut project_language)
        .expect("Failed to read line");

    match project_language.trim() {
        "1" => {
            println!("Select C standard:\n1. C11\n2. C17");
            io::stdin()
                .read_line(&mut standard)
                .expect("Failed to read line");

            match standard.trim() {
                "1" => {
                    project = project
                        .language(Language::C)
                        .standard(Standard::C11)
                        .build_command("clang -std=c11");
                }
                "2" => {
                    project = project
                        .language(Language::C)
                        .standard(Standard::C17)
                        .build_command("clang -std=c17");
                }
                _ => {
                    panic!("Invalid input");
                }
            }
        }

        "2" => {
            println!("Select C++ standard:\n1. C++11\n2. C++14\n3. C++17\n4. C++2a");
            io::stdin()
                .read_line(&mut standard)
                .expect("Failed to read line");
            match standard.trim() {
                "1" => {
                    project = project
                        .language(Language::CXX)
                        .standard(Standard::CXX11)
                        .build_command("clang++ -std=c++11");
                }
                "2" => {
                    project = project
                        .language(Language::CXX)
                        .standard(Standard::CXX14)
                        .build_command("clang++ -std=c++14")
                }
                "3" => {
                    project = project
                        .language(Language::CXX)
                        .standard(Standard::CXX17)
                        .build_command("clang++ -std=c++17");
                }
                "4" => {
                    project = project
                        .language(Language::CXX)
                        .standard(Standard::CXX2a)
                        .build_command("clang++ -std=c++2a")
                }
                _ => {
                    panic!("Invalid input");
                }
            }
        }
        _ => {
            panic!("Invalid input");
        }
    }

    return project;
}
