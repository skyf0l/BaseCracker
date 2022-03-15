use std::path::Path;

use clap::Parser;
use serde_json::json;

use basecracker::{self, modules::Base};

/// Convert result of basecracker in json format:
///
/// ```json
/// {
///   "cipher": ...,
///   "plaintexts": [
///     {
///       "plaintext": ...,
///       "bases": [
///         ...,
///         ...
///       ]
///     },
///     ...
///   ]
/// }
/// ```
fn plaintexts_to_json(cipher: &str, plaintexts: &Vec<(String, Vec<String>)>) -> String {
    let mut plaintexts_json = vec![];
    for (plaintext, bases) in plaintexts {
        plaintexts_json.push(json!({
            "plaintext": plaintext,
            "bases": json!(bases)
        }));
    }
    json!({
        "cipher": cipher,
        "plaintexts": json!(plaintexts_json)
    })
    .to_string()
}

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Encode given plaintext/file using the specified bases
    #[clap(
        short,
        long,
        value_name = "PLAINTEXT",
        conflicts_with_all = &["decode", "crack", "list"]
    )]
    encode: Option<String>,

    /// Decode given cipher/file using the specified bases
    #[clap(
        short,
        long,
        value_name = "CIPHER",
        conflicts_with_all = &["encode", "crack", "list"]
    )]
    decode: Option<String>,

    /// Crack the cipher/file using the specified bases (default: all)
    #[clap(
        short,
        long,
        value_name = "CIPHER",
        conflicts_with_all = &["encode", "decode", "list"]
    )]
    crack: Option<String>,

    /// Set base to use (can be separated by comma or space)
    #[clap(
        short,
        long,
        min_values = 1,
        requires_all = &["encode", "decode", "crack"]
    )]
    bases: Option<Vec<String>>,

    /// Reverse bases order (default: false)
    #[clap(
        short,
        long,
        requires_all = &["encode", "decode", "bases"]
    )]
    reversed: bool,

    /// List supported bases
    #[clap(
        short,
        long,
        conflicts_with_all = &["encode", "decode", "crack"],
    )]
    list: bool,

    /// Output cracker results in json format
    #[clap(
        short,
        long,
        requires = "crack",
        conflicts_with_all = &["verbose", "quiet"]
    )]
    json: bool,

    /// Verbose mode, print encoding/decoding steps
    #[clap(short, long, conflicts_with_all = &["quiet", "json"])]
    verbose: bool,

    /// Quiet mode, don't print anything except results
    #[clap(short, long, conflicts_with_all = &["verbose", "json"])]
    quiet: bool,
}

fn subcommand_list() {
    println!("Supported bases are:");
    for (short, long) in basecracker::get_bases_names() {
        println!("  - {:15}({})", long, short);
    }
}

fn parse_bases(bases: &Vec<String>, reversed: bool) -> Result<Vec<Box<dyn Base>>, String> {
    // split bases by comma or space
    let mut bases = bases
        // split by comma
        .iter()
        .flat_map(|base| base.split(',').map(|b| b.to_string()))
        .collect::<Vec<String>>()
        // split by space
        .iter()
        .flat_map(|base| base.split(' ').map(|b| b.to_string()))
        .collect::<Vec<String>>()
        // remove empty bases
        .iter()
        .map(|base| base.trim())
        .filter(|base| !base.is_empty())
        .map(|base| base.to_string())
        .collect::<Vec<String>>();
    // reverse bases order if needed
    if reversed {
        bases.reverse();
    }
    basecracker::get_bases_from_names(&bases)
}

// If argument is a file, read it, else return argument as is
fn get_file_content(arg: &str) -> Result<String, String> {
    if Path::new(arg).exists() {
        let content = std::fs::read_to_string(arg).map_err(|e| e.to_string())?;
        Ok(content)
    } else {
        Ok(arg.to_string())
    }
}

fn subcommand_encode(
    plaintext: &str,
    specified_bases: &Vec<Box<dyn Base>>,
    _arg_verbose: bool,
) -> Result<(), String> {
    // get plaintext from file or argument
    let plaintext = get_file_content(&plaintext)?;

    // encode
    let cipher = basecracker::encode(&plaintext, specified_bases)?;
    println!("{}", cipher);
    Ok(())
}

fn subcommand_decode(
    cipher: &str,
    specified_bases: &Vec<Box<dyn Base>>,
    _arg_verbose: bool,
) -> Result<(), String> {
    // get cipher from file or argument
    let cipher = get_file_content(&cipher)?;

    // decode
    let plaintext = basecracker::decode(&cipher, specified_bases)?;
    println!("{}", plaintext);
    Ok(())
}

fn subcommand_crack(
    cipher: &str,
    specified_bases: &Option<Vec<Box<dyn Base>>>,
    _arg_verbose: bool,
    arg_json: bool,
    arg_quiet: bool,
) -> Result<(), String> {
    // get cipher from file or argument
    let cipher = get_file_content(&cipher)?;

    // crack subcommand
    let plaintexts = match specified_bases {
        Some(bases) => basecracker::basecracker_with_bases(&cipher, &bases),
        None => basecracker::basecracker(&cipher),
    };

    if arg_json {
        // output in json format
        println!("{}", plaintexts_to_json(&cipher, &plaintexts));
        // if no plaintexts were found, exit with error code
        if plaintexts.len() == 0 {
            std::process::exit(1);
        }
    } else {
        // output in plaintext format
        if plaintexts.len() != 0 {
            for (plaintext, bases) in plaintexts {
                if !arg_quiet {
                    println!("Recipe: {}", bases.join(" -> "));
                    println!("Result: {}", plaintext);
                } else {
                    println!("{}", plaintext);
                }
            }
        } else {
            // if no plaintexts were found, display error and exit with error code
            if !arg_quiet {
                eprintln!("No plaintexts found");
            }
            std::process::exit(1);
        }
    }
    Ok(())
}

fn main() {
    let args = Args::parse();

    if args.list {
        // list subcommand
        subcommand_list();
    } else {
        // parse bases from args
        let specified_bases = match args.bases {
            Some(bases) => match parse_bases(&bases, args.reversed) {
                Ok(bases) => Some(bases),
                Err(err) => {
                    eprintln!("{}", err);
                    std::process::exit(1);
                }
            },
            None => None,
        };

        match {
            if let Some(cipher) = args.crack {
                // call crack subcommand
                subcommand_crack(
                    &cipher,
                    &specified_bases,
                    args.verbose,
                    args.json,
                    args.quiet,
                )
            } else {
                // get bases
                let specified_bases = match specified_bases {
                    Some(bases) => bases,
                    None => {
                        eprintln!("No bases specified");
                        eprintln!("Use --bases to specify bases");
                        std::process::exit(1);
                    }
                };

                // call encode or decode subcommand
                if let Some(plaintext) = args.encode {
                    subcommand_encode(&plaintext, &specified_bases, args.verbose)
                } else if let Some(cipher) = args.decode {
                    subcommand_decode(&cipher, &specified_bases, args.verbose)
                } else {
                    // no subcommand specified
                    Err(String::from(
                        "No subcommand specified\nFor more information try --help",
                    ))
                }
            }
        } {
            Ok(_) => (),
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        }
    }
}
