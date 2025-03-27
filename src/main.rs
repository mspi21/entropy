#[derive(Debug)]
enum Error {
    FileOpenError,
    UnexpectedIOError,
}

fn main() {
    let mut args = std::env::args();
    let _ = args.next().unwrap();

    let mut files_processed = 0;

    for arg in args {
        if arg == "--help" || arg == "-h" {
            print_help();
            std::process::exit(0);
        }
        let path = std::path::Path::new(&arg);
        let entropy = match shannon_entropy_of_file(path) {
            Ok(entropy) => entropy,
            Err(Error::FileOpenError) => {
                eprintln!(
                    "Could not open file '{}' for reading, skipping.",
                    path.to_str().unwrap()
                );
                continue;
            }
            Err(Error::UnexpectedIOError) => {
                eprintln!(
                    "Got unexpected IO error reading from file '{}', skipping.",
                    path.to_str().unwrap()
                );
                continue;
            }
        };
        print_result(Some(path), entropy);
        files_processed += 1;
    }

    if files_processed == 0 {
        let entropy = shannon_entropy(&mut std::io::stdin())
            .expect("Got unexpected IO error reading from stdin.");
        print_result(None, entropy);
    }
}

fn print_help() {
    const HELP: &str = r#"Usage: entropy [FILE]...
        Compute the Shannon entropy of FILE(s).
        With no FILE, read standard input."#;
    eprintln!("{}", HELP);
}

fn print_result(target: Option<&std::path::Path>, measured_quantity: f64) {
    if let Some(path) = target {
        print!("{}: ", path.to_str().unwrap());
    }
    println!("{}", measured_quantity);
}

fn shannon_entropy(stream: &mut impl Read) -> Result<f64, Error> {
    let mut buf: [u8; 4096] = [0; 4096];
    let mut absolute_frequencies: [usize; 256] = [0; 256];

    let mut total_size: usize = 0;

    loop {
        let read = stream
            .read(&mut buf)
            .map_err(|_| Error::UnexpectedIOError)?;

        if read == 0 {
            break;
        }

        for i in 0..read {
            absolute_frequencies[buf[i] as usize] += 1;
        }

        total_size += read;
    }

    Ok(-absolute_frequencies
        .into_iter()
        .map(|frequency| (frequency as f64) / (total_size as f64))
        .map(|probability| probability * probability.log2())
        .map(|e_or_nan| if e_or_nan.is_nan() { 0f64 } else { e_or_nan })
        .sum::<f64>())
}

fn shannon_entropy_of_file(filepath: &std::path::Path) -> Result<f64, Error> {
    let mut file = std::fs::File::open(filepath).map_err(|_| Error::FileOpenError)?;
    shannon_entropy(&mut file)
}

use std::io::Read;
