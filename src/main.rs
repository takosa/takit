use structopt::StructOpt;
mod lib;
<<<<<<< HEAD
pub use crate::lib::{vcflank, randfa, randsub};
=======
pub use crate::lib::vcflank;
pub use crate::lib::randfa;
pub use crate::lib::snpdensity;
>>>>>>> 82976a2e41423a17d316748ddef8a60991863483

#[derive(StructOpt)]
enum Cli {
    Vcflank(vcflank::VcflankOpt),
    Randfa(randfa::RandfaOpt),
<<<<<<< HEAD
    Randsub(randsub::RandsubOpt),
=======
    Snpdensity(snpdensity::SnpdensityOpt),
>>>>>>> 82976a2e41423a17d316748ddef8a60991863483
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
<<<<<<< HEAD
        Cli::Randsub(args) => {
            randsub::randsub(args);
=======
        Cli::Snpdensity(args) => {
            snpdensity::snpdensity(args);
>>>>>>> 82976a2e41423a17d316748ddef8a60991863483
        },
    }
}
