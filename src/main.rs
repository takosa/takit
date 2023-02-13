use structopt::StructOpt;
mod lib;
pub use crate::lib::{vcflank, randfa, randsub};

#[derive(StructOpt)]
enum Cli {
    Vcflank(vcflank::VcflankOpt),
    Randfa(randfa::RandfaOpt),
    Randsub(randsub::RandsubOpt),
}

fn main() {
    let args = Cli::from_args();
    match args {
        Cli::Vcflank(args) => {
            vcflank::vcflank(args);
        },
        Cli::Randfa(args) => {
            randfa::randfa(args);
        },
        Cli::Randsub(args) => {
            randsub::randsub(args);
        },
    }
}
