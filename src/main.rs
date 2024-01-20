mod handlers;
use handlers::comment::execute_cmt;
use clap::{App, Arg, SubCommand};



//Commands the client can request.

fn main() {
    let matches = App::new("ACM")
        .version("1.0.0")
        .author("SF Hacks @ CruzHacks ^-^")
        .about("This command tool allows you to use GPT to modify your existing project and add comments")
        .subcommand(
            SubCommand::with_name("prs")
                .about("Reads the files specified in the path and creates slide document ")
                .arg(Arg::with_name("dir").required(true).help("The name of the directory comments are wanted on").default_value("./")).arg(Arg::with_name("type").help("The file type to add comments to")),
        )
        .subcommand(
            SubCommand::with_name("gc")
                .about("Adds a git commit")
               
        )
        .get_matches();

    match matches.subcommand(){
        ("prs", Some(cmd)) => {
            let dir = cmd.value_of("dir").unwrap();
            let file_type = cmd.value_of("type").unwrap_or("all");
            execute_cmt(dir
                , file_type)
            
          
        },_ => {
            println!("No or unknown subcommand provided.");
        }
    }
}
