// take user input for project name

use terminal_menu::{button, label, menu, mut_menu, run};

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

impl Into<String> for Language {
    fn into(self) -> String {
        match self {
            Language::C => "C".to_string(),
            Language::CXX => "CXX".to_string(),
        }
    }
}

impl Into<String> for Standard {
    fn into(self) -> String {
        match self {
            Standard::C11 => "C11".to_string(),
            Standard::C17 => "C17".to_string(),
            Standard::CXX11 => "CXX11".to_string(),
            Standard::CXX14 => "CXX14".to_string(),
            Standard::CXX17 => "CXX17".to_string(),
            Standard::CXX2a => "CXX2a".to_string(),
        }
    }
}

trait EnumIterator: Sized {
    fn variants() -> Vec<Self>;

    fn into_iter(self) -> std::vec::IntoIter<Self> {
        Self::variants().into_iter()
    }
}

impl EnumIterator for Language {
    fn variants() -> Vec<Self> {
        vec![Language::C, Language::CXX]
    }
}

impl EnumIterator for Standard {
    fn variants() -> Vec<Self> {
        vec![
            Standard::C11,
            Standard::C17,
            Standard::CXX11,
            Standard::CXX14,
            Standard::CXX17,
            Standard::CXX2a,
        ]
    }
}

impl Language {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "C" => Some(Language::C),
            "CXX" => Some(Language::CXX),
            _ => None,
        }
    }
}

impl Standard {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "C11" => Some(Standard::C11),
            "C17" => Some(Standard::C17),
            "CXX11" => Some(Standard::CXX11),
            "CXX14" => Some(Standard::CXX14),
            "CXX17" => Some(Standard::CXX17),
            "CXX2a" => Some(Standard::CXX2a),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub language: Option<Language>,
    pub standard: Option<Standard>,
    pub build_command: Option<String>,
}

impl Project {
    pub fn new(name: &str) -> Project {
        Project {
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

pub fn language_menu() -> Option<Language> {
    let menu = menu(vec![label("Choose language:")]);

    /* To enable creation of menu without buttons, src code for terminal_menu::menu()
    needed modification (see: https://gitlab.com/xamn/terminal-menu-rs/-/issues/4)
    */

    for language in Language::variants() {
        menu.write().unwrap().items.push(button(language));
    }
    menu.write().unwrap().set_selected_item_with_index(1);

    run(&menu);
    let language = mut_menu(&menu).selected_item_name().to_owned();
    // println!("Language: {}", language);

    Language::from_str(&language)
}
pub fn standard_menu() -> Option<Standard> {
    let menu = menu(vec![label("Choose standard:")]);
    for standard in Standard::variants() {
        menu.write().unwrap().items.push(button(standard));
    }
    menu.write().unwrap().set_selected_item_with_index(1);
    run(&menu);

    let standard = mut_menu(&menu).selected_item_name().to_owned();
    // println!("Standard: {}", standard);

    Standard::from_str(&standard)
}

// prompt user and return input
pub fn prompt(msg: &str) -> String {
    println!("{}", msg);
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input.trim().to_string();
}

pub fn initialize_project() -> Project {
    let name: String = prompt("Enter project name: ");
    let language = language_menu();
    let standard = standard_menu();

    let mut project = Project {
        name,
        language,
        standard,
        build_command: None,
    };

    return project;
}

/* println!("Choose project language:\n1. C\n2. C++");
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
} */
