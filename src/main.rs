extern create flate2;

use flate2::writer::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::env::args::Array;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;



fn main() {
    println!("Hello, world!");

    if args().len() != 3 {
        eprint!("Usage: `ource` `target`");
        return;
    }

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());
}
