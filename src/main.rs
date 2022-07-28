use structopt::StructOpt;
mod lib;
pub use crate::lib::vcflank;
pub use crate::lib::randfa;
pub use crate::lib::snpdensity;

#[derive(StructOpt)]
enum Cli {
    Vcflank(vcflank::VcflankOpt),
    Randfa(randfa::RandfaOpt),
    Snpdensity(snpdensity::SnpdensityOpt),
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
    }
}
