pub mod metadata {
    use serde::{Deserialize, Serialize};
    use std::{error::Error, fs::File, io::Read};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct OpOneMetadata {
        drum_version: u8,
        dyna_env: Vec<u64>,
        end: Vec<u64>,
        fx_active: bool,
        fx_params: Vec<u64>,
        fx_type: String,
        lfo_active: bool,
        lfo_params: Vec<u64>,
        lfo_type: String,
        name: String,
        octave: u64,
        pitch: Vec<u64>,
        playmode: Vec<u64>,
        start: Vec<u64>,
        reverse: Vec<u64>,
        r#type: String,
        volume: Vec<u64>,
    }

    impl OpOneMetadata {
        pub fn parse(filename: &str) -> Result<Self, Box<dyn Error>> {
            let input_file = File::open(filename).expect("Cannot open input file");
            let bytes: Vec<u8> = input_file.bytes().map(|x| x.unwrap()).collect();

            // "op-1"
            const OP1: &'static [u8] = &[0x6f, 0x70, 0x2d, 0x31];
            // "SSND" 53534E44
            const SSND: &'static [u8] = &[0x53, 0x53, 0x4e, 0x44];

            let byte_iter = bytes.windows(4);
            let mut start = None;

            for (i, chunk) in byte_iter.enumerate() {
                if chunk == OP1 {
                    start = Some(i + 4); // start after "op-1"
                }

                if let Some(s) = start {
                    if chunk == SSND && s < i {
                        let parsed: OpOneMetadata = serde_json::from_slice(&bytes[s..i])?;
                        return Ok(parsed);
                    }
                }
            }

            Err("Failed to parse metadata".into())
        }
    }
}
