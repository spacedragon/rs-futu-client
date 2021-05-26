use std::io::Result;
use std::{fs, io};

fn main() -> Result<()> {
    let entries = fs::read_dir("./protos")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>>>()?;

    prost_build::Config::default()
        .out_dir("./src/protos")
        .compile_protos(entries.as_slice(), &[std::path::PathBuf::from("./protos")])?;
    Ok(())
}