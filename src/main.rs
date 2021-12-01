use basecracker;

fn main() {
    // get args
    let args: Vec<String> = std::env::args().collect();

    // check args
    if args.len() != 2 {
        println!("Usage: {} <cipher>", args[0]);
        return;
    }
    basecracker::basecracker(&args[1]);
}
