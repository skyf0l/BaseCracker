use basecracker;

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

fn main() {
    // get args
    let args: Vec<String> = std::env::args().collect();

    // check args
    if args.len() != 2 {
        println!("Usage: {} <cipher>", args[0]);
        return;
    }
    let plaintexts = basecracker::basecracker(&args[1]);
    println!("{}", plaintexts_to_json(&args[1], &plaintexts));
    for (plaintext, bases) in plaintexts {
        println!("Decode order: {}", bases.join(" -> "));
        println!("Plaintext: {}", plaintext);
    }
}
