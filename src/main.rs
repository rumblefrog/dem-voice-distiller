use std::path::PathBuf;

use std::fs::File;

use std::io::prelude::*;

use structopt::StructOpt;

mod demo_file;
mod demo_header;
mod errors;

use errors::Result;

use demo_file::DemoFile;

#[derive(StructOpt)]
struct Cli {
    #[structopt(name = "file", parse(from_os_str))]
    /// DEM file to distill
    pub file: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::from_args();

    let mut file = File::open(cli.file)?;

    let mut contents = Vec::new();

    file.read_to_end(&mut contents)?;

    let mut dem = DemoFile::new(contents);

    let header = dem.header()?;

    println!("{:?}", header);

    Ok(())
}
