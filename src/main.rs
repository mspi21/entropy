#[derive(Debug)]
enum Error {
    FileNotOpenable,
    PathNotFound,
    NotARegularFile,
    UnexpectedIO,
}

fn main() {
    let mut args = std::env::args();
    let _ = args.next().unwrap();

    let args: Vec<String> = args.collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        std::process::exit(0);
    }

    process_filelist(args);
}

fn process_filelist(filepaths: Vec<String>) {
    if filepaths.is_empty() {
        let entropy = shannon_entropy(&mut std::io::stdin())
            .expect("Got unexpected IO error reading from stdin.");
        print_result(None, entropy, "bits");
    }

    let path_iter = filepaths
        .iter()
        .map(|f| if f == "-" { "/dev/stdin" } else { f })
        .map(std::path::Path::new);

    for path in path_iter {
        let entropy = match shannon_entropy_of_file(path) {
            Ok(entropy) => entropy,
            Err(Error::PathNotFound) => {
                eprintln!(
                    "The specified path '{}' does not exist, skipping.",
                    path.to_str().unwrap()
                );
                continue;
            }
            Err(Error::NotARegularFile) => {
                eprintln!(
                    "The specified file '{}' is not a regular file, skipping.",
                    path.to_str().unwrap()
                );
                continue;
            }
            Err(Error::FileNotOpenable) => {
                eprintln!(
                    "Could not open file '{}' for reading, skipping.",
                    path.to_str().unwrap()
                );
                continue;
            }
            Err(Error::UnexpectedIO) => {
                eprintln!(
                    "Got unexpected IO error reading from file '{}', skipping.",
                    path.to_str().unwrap()
                );
                continue;
            }
        };
        print_result(Some(path), entropy, "bits");
    }
}

fn print_help() {
    const HELP: &str = r#"Usage: entropy [FILE]...
Compute the (Shannon) entropy of the octet distribution in FILE(s).

With no FILE, or when FILE is -, read standard input.
Otherwise, FILE must be a regular file."#;

    eprintln!("{}", HELP);
}

fn print_result(target: Option<&std::path::Path>, measured_quantity: f64, measured_unit: &str) {
    if let Some(path) = target {
        print!("{}: ", path.to_str().unwrap());
    }
    println!("{} {}", measured_quantity, measured_unit);
}

fn shannon_entropy(stream: &mut impl Read) -> Result<f64, Error> {
    let mut buf: [u8; 4096] = [0; 4096];
    let mut histogram: [usize; 256] = [0; 256];

    let mut total_size: usize = 0;

    loop {
        let read = stream.read(&mut buf).map_err(|_| Error::UnexpectedIO)?;

        if read == 0 {
            break;
        }

        for i in 0..read {
            histogram[buf[i] as usize] += 1;
        }

        total_size += read;
    }

    let entropy = -histogram
        .into_iter()
        .map(|frequency| (frequency as f64) / (total_size as f64))
        .map(|probability| probability * probability.log2())
        .map(|e_or_nan| if e_or_nan.is_nan() { 0f64 } else { e_or_nan })
        .sum::<f64>();

    // Convert -0 to 0.
    Ok(f64::max(0f64, entropy))
}

fn shannon_entropy_of_file(filepath: &std::path::Path) -> Result<f64, Error> {
    if !filepath.exists() {
        return Err(Error::PathNotFound);
    }

    if !filepath.is_file() && filepath != std::path::Path::new("/dev/stdin") {
        return Err(Error::NotARegularFile);
    }

    let mut file = std::fs::File::open(filepath).map_err(|_| Error::FileNotOpenable)?;
    shannon_entropy(&mut file)
}

use std::io::Read;
