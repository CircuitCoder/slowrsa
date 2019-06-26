use slowrsa::key;

#[derive(structopt::StructOpt)]
struct Args {
    /// N
    #[structopt(short = "n")]
    n: String,

    /// E
    #[structopt(short = "e")]
    e: String,

    /// M, in hex form
    #[structopt(short = "m")]
    m: String,
}

#[paw::main]
fn main(args: Args) {
    let pubkey = key::PubKey::from_str(&args.n, &args.e).unwrap();
    let c = pubkey.encrypt(&args.m).unwrap();
    println!("{}", c);
}
