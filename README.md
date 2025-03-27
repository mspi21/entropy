# entropy &mdash; a minimal GNU-like tool for calculating entropy of files

Because how isn't there already one (in my favourite package manager)?

## Installation (from source)

Prerequisites:
- An up-to-date Rust toolchain (2024 edition).

Installation steps:

1. Review the source code and dependencies for backdoors, malware and zero-day exploits.
2. Clone the repo.
3. Run `cargo install --path .` to install `entropy` locally for the current user.
4. ???
5. Profit!

## Usage

See `entropy --help`.

## Possible future features

- `-r` option to recursive traverse directories
- `--algorithm` option to specify other algorithms besides Shannon's entropy
- `--pretty-print` option to print a nice table with the results and explanations
