use std::fs::write;

use clap::{App, Arg, SubCommand};
use rustyline::{Editor, Result};

fn main() {
    let app = App::new("lanner")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("run").about("run a program").arg(
                Arg::with_name("INPUT")
                    .takes_value(true)
                    .required(false)
                    .help("the file to run"),
            ),
        )
        .subcommand(
            SubCommand::with_name("doc")
                .about("document your package")
                .arg(Arg::with_name("name").takes_value(true).required(true)),
        )
        .subcommand(
            SubCommand::with_name("repl")
                .about("run charon on the command line")
                .arg(Arg::with_name("name").takes_value(true).required(true)),
        )
        .subcommand(
            SubCommand::with_name("check")
                .about("validate your charon package")
                .arg(Arg::with_name("name").takes_value(true).required(true)),
        )
        .subcommand(
            SubCommand::with_name("help")
                .about("get help with charon")
                .arg(Arg::with_name("name").takes_value(true).required(true)),
        )
        .subcommand(SubCommand::with_name("new").about("create a new charon package"))
        .get_matches();

    if let Some(v) = app.subcommand_matches("new") {
        let mut rl = Editor::<()>::new();
        let config = vec!["name", "description", "version", "license", "source"];
        let mut res: Vec<String> = vec![];
        for i in config {
            res.push(rl.readline(format!("{}: ", i).as_str()).unwrap());
        }
        write(
            "pkg.plto",
            format!(
                "name \"{}\"\ndescription \"{}\"\nversion \"{}\"\nlicense \"{}\"\nsource \"{}\"\n",
                res[0], res[1], res[2], res[3], res[4]
            ),
        )
        .unwrap();
    }
}
