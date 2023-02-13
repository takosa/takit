use structopt::StructOpt;
mod lib;
pub use crate::lib::{vcflank, randfa, snpdensity, randsub};

#[derive(StructOpt)]
enum Cli {
    Vcflank(vcflank::VcflankOpt),
    Randfa(randfa::RandfaOpt),
    Snpdensity(snpdensity::SnpdensityOpt),
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
        Cli::Snpdensity(args) => {
            snpdensity::snpdensity(args);
        },
        Cli::Randsub(args) => {
            randsub::randsub(args);
        },
    }
}
