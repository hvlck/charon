use structopt::{clap::AppSettings, StructOpt};

#[derive(StructOpt, Debug)]
#[structopt(name = "lc", about = "lightning compiler", setting = AppSettings::InferSubcommands)]
enum Opt {
    Build,
    Run,
    Doc,
    Repl,
    Check,
    Help,
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Build => {}
        Opt::Repl => {}
        _ => {}
    };
}
