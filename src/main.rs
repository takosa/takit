use structopt::StructOpt;
use takit::*;

#[derive(StructOpt)]
enum Cli {
    Vcflank(vcflank::VcflankOpt),
    Randfa(randfa::RandfaOpt),
    Snpdensity(snpdensity::SnpdensityOpt),
    Randsub(randsub::RandsubOpt),
    Abc(abc::AbcOpt),
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
        Cli::Abc(args) => {
            abc::abc(args);
        },
    }
}
