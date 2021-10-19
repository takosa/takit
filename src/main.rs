use structopt::StructOpt;
mod lib;
pub use crate::lib::vcflank;

#[derive(StructOpt)]
enum Cli {
    Vcflank(vcflank::VcflankOpt),
}

fn main() {
    let args = Cli::from_args();
    match args {
        Cli::Vcflank(args) => {
            vcflank::vcflank(args);
        },
    }
}
