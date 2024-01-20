mod handlers;
use handlers::comment::execute_cmt;
use clap::{App, Arg, SubCommand};
use std::env;


//Commands the client can request.

fn main() {
    let matches = App::new("ACM")
        .version("1.0")
        .author("SF Hacks @ CruzHacks ^-^")
        .about("This command tool allows you to use GPT to modify your existing project and add comments")
        .subcommand(
            SubCommand::with_name("cmt")
                .about("Adds comments to the existing directory")
                .arg(Arg::with_name("type").help("The file type to add comments to")).arg(Arg::with_name("dir").required(false).help("The name of the directory comments are wanted on")),
        )
        .subcommand(
            SubCommand::with_name("gc")
                .about("Adds a git commit")
               
        )
        .get_matches();

    match matches.subcommand(){
        ("cmt", Some(cmd)) => {
            let file_type = cmd.value_of("type").unwrap_or("all");

            if let Ok(current_dir) = env::current_dir() {
              execute_cmt(current_dir.to_str().unwrap_or("none"), file_type)
            } else {
                eprintln!("Failed to get the current working directory.");
            }
        },_ => {
            println!("No or unknown subcommand provided.");
        }
    }
}
