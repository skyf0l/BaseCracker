use clap::{command, Parser, Subcommand};
use main_error::MainError;
use std::path::Path;
use std::{io, io::Write};

use basecracker::{crack, decode, encode, get_recipe, BaseError, DecodeError};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
struct Args {
    #[clap(flatten)]
    options: Options,
    /// Subcommands.
    #[command(subcommand)]
    subcommand: SubCommand,
}

/// Options.
#[derive(Parser, Debug, Clone)]
struct Options {
    /// Quiet mode, don't print anything except results
    #[clap(short, long, conflicts_with = "verbose")]
    quiet: bool,
    /// Verbose mode
    #[clap(short, long, conflicts_with = "quiet")]
    verbose: bool,
    /// Minimum printable percentage to consider a result valid
    #[clap(short, long, default_value = "0.9")]
    min_printable_percentage: f32,
    /// Do not output the trailing newline
    #[clap(short, long)]
    no_newline: bool,
}

/// Subcommands.
#[derive(Subcommand, Debug, Clone)]
enum SubCommand {
    /// Encode given plaintext/file using the specified bases
    Encode {
        /// The plaintext to encode (can be a file)
        plaintext: String,
        /// The bases to use (can be separated by comma or space)
        bases: Bases,
        /// Reverse the order of the bases
        #[clap(short, long)]
        reverse: bool,
    },
    /// Decode given cipher/file using the specified bases
    Decode {
        /// The cipher to decode (can be a file)
        ciphertext: String,
        /// The bases to use (can be separated by comma or space)
        bases: Bases,
        /// Reverse the order of the bases
        #[clap(short, long)]
        reverse: bool,
    },
    /// Crack given cipher/file
    Crack {
        /// The cipher to crack (can be a file)
        ciphertext: String,
    },
}

#[derive(Debug, Clone)]
struct Bases(Vec<String>);

impl std::str::FromStr for Bases {
    type Err = String;

    #[cfg(not(tarpaulin_include))]
    fn from_str(bases: &str) -> Result<Self, Self::Err> {
        let bases = bases
            // Split by comma
            .split(',')
            .map(|base| base.to_string())
            .collect::<Vec<String>>()
            // Split by space
            .iter()
            .flat_map(|base| base.split(' ').map(|b| b.to_string()))
            .collect::<Vec<String>>()
            // Remove empty bases
            .iter()
            .map(|base| base.trim())
            .filter(|base| !base.is_empty())
            .map(|base| base.to_string())
            .collect::<Vec<String>>()
            // Check if all bases are valid
            .iter()
            .map(|base| {
                if basecracker::get_base_from_name(base).is_err() {
                    Err(format!("Invalid base: {}", base))
                } else {
                    Ok(base.to_string())
                }
            })
            .collect::<Result<Vec<String>, String>>()?;

        // Check if there is at least one base
        if bases.is_empty() {
            Err("No base specified".to_string())
        } else {
            Ok(())
        }?;

        Ok(Bases(bases))
    }
}

/// If argument is a file, read it and return its content, else return the argument as is
#[cfg(not(tarpaulin_include))]
fn read_file_or_arg(arg: String) -> String {
    if Path::new(&arg).exists() {
        std::fs::read_to_string(arg).unwrap()
    } else {
        arg
    }
}

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
enum Error {
    #[error(transparent)]
    DecodeError(#[from] DecodeError),
    #[error(transparent)]
    BaseError(#[from] BaseError),
}

#[cfg(not(tarpaulin_include))]
fn display_result(result: &[u8], options: &Options) -> std::io::Result<()> {
    if options.no_newline {
        io::stdout().write_all(result)?;
    } else {
        io::stdout().write_all(result)?;
        io::stdout().write_all(b"\n")?;
    }
    Ok(())
}

#[cfg(not(tarpaulin_include))]
fn main() -> Result<(), MainError> {
    let args = Args::parse();

    match args.subcommand {
        SubCommand::Encode {
            plaintext,
            bases,
            reverse,
        } => {
            let plaintext = read_file_or_arg(plaintext);
            let mut bases = basecracker::get_bases_from_names(&bases.0)?;
            if reverse {
                bases.reverse();
            }

            let result = encode(&plaintext, &bases);
            display_result(result.last().unwrap().as_bytes(), &args.options)?;
        }
        SubCommand::Decode {
            ciphertext,
            bases,
            reverse,
        } => {
            let ciphertext = read_file_or_arg(ciphertext);
            let mut bases = basecracker::get_bases_from_names(&bases.0)?;
            if reverse {
                bases.reverse();
            }

            let result = decode(&ciphertext, &bases)?;
            display_result(result.last().unwrap(), &args.options)?;
        }
        SubCommand::Crack { ciphertext } => {
            let ciphertext = read_file_or_arg(ciphertext);

            let result = crack(
                &ciphertext,
                &basecracker::get_bases(),
                args.options.min_printable_percentage,
            );
            let leaves = result.leaves();

            if leaves.is_empty() {
                // No result found
                eprintln!("Error: No result found");
            } else if leaves.len() == 1 {
                // One result found (no ambiguity)
                let leaf = leaves[0].clone();
                eprintln!("Recipe: {}", get_recipe(&leaf));
                display_result(&leaf.borrow().data.decoded, &args.options)?;
            } else {
                // Multiple results found
                eprintln!("Warning: {} results found, you may want to use the --min-printable-percentage option", leaves.len());
                for leaf in leaves {
                    println!("Recipe: {}", get_recipe(&leaf));
                    io::stdout().write_all(&leaf.borrow().data.decoded)?;
                    println!();
                }
            }
        }
    }

    Ok(())
}
