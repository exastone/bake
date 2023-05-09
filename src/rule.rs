use std::{
    fs::{self, File},
    io::Write,
};

// GNU Makefile rule
pub struct Rule {
    pub target: String,
    pub prereqs: Vec<String>,
    pub recipe: Vec<String>,
}

pub fn create_rule(target: String, prereqs: Vec<String>, recipe: Vec<String>) -> Rule {
    Rule {
        target,
        prereqs,
        recipe,
    }
}

pub fn write_rule(file: &mut File, rule: &Rule) {
    // write target and prereqs to file
    file.write_all(format!("{}: {}\n", rule.target, rule.prereqs.join(" ")).as_bytes())
        .err();
    // write recipe to file
    file.write_all(format!("\t{}\n", rule.recipe.join(" ")).as_bytes())
        .err();
}
