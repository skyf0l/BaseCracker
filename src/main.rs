use basecracker;
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
        conflicts_with_all = &["decode", "crack"]
    )]
    encode: Option<String>,

    /// Decode given cipher using the specified bases
    #[clap(
        short,
        long,
        value_name = "CIPHER",
        conflicts_with_all = &["encode", "crack"]
    )]
    decode: Option<String>,

    /// Try to crack the cipher using the specified bases (default: all)
    #[clap(
        short,
        long,
        value_name = "CIPHER",
        conflicts_with_all = &["encode", "decode"]
    )]
    crack: Option<String>,

    /// Set base to use for encoding/decoding
    #[clap(
        short,
        long, 
        min_values = 1,
        requires_all = &["encode", "decode", "crack"]
    )]
    bases: Option<Vec<String>>,

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

fn main() {
    let args = Args::parse();

    if let Some(_plaintext) = args.encode {
        let _bases = args.bases;
        todo!();
    } else if let Some(_cipher) = args.decode {
        let _bases = args.bases;
        todo!()
    } else if let Some(cipher) = args.crack {
        let _bases = args.bases;
        let plaintexts = basecracker::basecracker(&cipher);

        if args.json {
            println!("{}", plaintexts_to_json(&cipher, &plaintexts));
            // if no plaintexts were found, exit with error code
            if plaintexts.len() == 0 {
                std::process::exit(1);
            }
        } else {
            if plaintexts.len() != 0 {
                for (plaintext, bases) in plaintexts {
                    if !args.quiet {
                        println!("Decode order: {}", bases.join(" -> "));
                        println!("Plaintext: {}", plaintext);
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
        eprintln!("No action specified");
        eprintln!("For more information try --help");
        std::process::exit(1);
    }
}
