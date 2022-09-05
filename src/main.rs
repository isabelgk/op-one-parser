use opone::metadata::OpOneMetadata;
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = env::args().nth(1).expect("usage: opone-cli FILE");

    let op1file: OpOneMetadata = OpOneMetadata::parse(&input_path)?;
    dbg!(op1file);

    Ok(())
}
