mod bake_cli;
mod bake_init;
mod bakefile;
mod helpers;

use bake_cli::{clap_cli, Cli};

fn main() {
    // let project: Project = bake_init::initialize_project();
    // println!("{:?}", project);

    let cli = <Cli as clap::Parser>::parse();
    clap_cli(cli)

    // Continued program logic goes here...
}
