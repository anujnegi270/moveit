use clap::{Arg, ArgMatches, Command};

pub fn get_app() -> ArgMatches {
    Command::new("MoveIt")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Anuj Negi <anujnegi@microsoft.com>")
        .about("Copies files to a specified folder")
        .arg(
            Arg::new("azure")
                .short('a')
                .long("azure")
                .help("To copy from your Azure Storage-xDpuWindows folder"),
        )
        .arg(
            Arg::new("target")
                .help("Sets the target folder name")
                .long("target")
                .short('t'),
        )
        .get_matches()
}
