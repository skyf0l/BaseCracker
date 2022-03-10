use basecracker::{self, modules::Base};
use clap::Parser;

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
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"cipher\": \"");
    json.push_str(cipher);
    json.push_str("\",\n");
    json.push_str("  \"plaintexts\": [\n");
    for (i, (plaintext, bases)) in plaintexts.iter().enumerate() {
        json.push_str("    {\n");
        json.push_str("      \"plaintext\": \"");
        json.push_str(plaintext);
        json.push_str("\",\n");
        json.push_str("      \"bases\": [\n");
        for (j, base) in bases.iter().enumerate() {
            json.push_str("        \"");
            json.push_str(base);
            json.push_str("\"");
            if j < bases.len() - 1 {
                json.push_str(",");
            }
            json.push_str("\n");
        }
        json.push_str("      ]\n");
        json.push_str("    }");
        if i < plaintexts.len() - 1 {
            json.push_str(",");
        }
        json.push_str("\n");
    }
    json.push_str("  ]\n");
    json.push_str("}");
    json
}

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Encode given plaintext using the specified bases
    #[clap(
        short,
        long,
        value_name = "PLAINTEXT",
        conflicts_with_all = &["decode", "crack", "list"]
    )]
    encode: Option<String>,

    /// Decode given cipher using the specified bases
    #[clap(
        short,
        long,
        value_name = "CIPHER",
        conflicts_with_all = &["encode", "crack", "list"]
    )]
    decode: Option<String>,

    /// Try to crack the cipher using the specified bases (default: all)
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

fn parse_bases(bases: &Option<Vec<String>>) -> Result<Vec<Box<dyn Base>>, String> {
    match bases {
        Some(bases) => {
            // split bases by comma or space
            let bases = bases
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
            basecracker::get_bases_from_names(&bases)
        }
        None => Err("No bases specified".to_string()),
    }
}

fn main() {
    let args = Args::parse();

    if args.list {
        // list subcommand
        subcommand_list();
    } else {
        // parse bases from args
        let specified_bases = match args.bases {
            Some(bases) => match parse_bases(&Some(bases)) {
                Ok(bases) => Some(bases),
                Err(err) => {
                    eprintln!("{}", err);
                    std::process::exit(1);
                }
            },
            None => None,
        };

        if let Some(_plaintext) = args.encode {
            // encode subcommand
            if let Some(_bases) = specified_bases {
                todo!();
            } else {
                eprintln!("No bases specified");
                eprintln!("Use --bases to specify bases");
            }
        } else if let Some(_cipher) = args.decode {
            // decode subcommand
            if let Some(_bases) = specified_bases {
                todo!();
            } else {
                eprintln!("No bases specified");
                eprintln!("Use --bases to specify bases");
            }
        } else if let Some(cipher) = args.crack {
            // crack subcommand
            let plaintexts = match specified_bases {
                Some(bases) => basecracker::basecracker_with_bases(&cipher, &bases),
                None => basecracker::basecracker(&cipher),
            };
            if args.json {
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
                        if !args.quiet {
                            println!("Recipe: {}", bases.join(" -> "));
                            println!("Result: {}", plaintext);
                        } else {
                            println!("{}", plaintext);
                        }
                    }
                } else {
                    // if no plaintexts were found, display error and exit with error code
                    if !args.quiet {
                        eprintln!("No plaintexts found");
                    }
                    std::process::exit(1);
                }
            }
        } else {
            // no subcommand specified
            eprintln!("No action specified");
            eprintln!("For more information try --help");
            std::process::exit(1);
        }
    }
}
