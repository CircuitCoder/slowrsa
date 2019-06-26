use slowrsa::key;

#[derive(structopt::StructOpt)]
struct Args {
    /// N
    #[structopt(short = "n")]
    n: String,

    /// D
    #[structopt(short = "d")]
    d: String,

    /// C, in hex form
    #[structopt(short = "c")]
    c: String,
}

#[paw::main]
fn main(args: Args) {
    let privkey = key::PrivKey::from_str(&args.n, &args.d).unwrap();
    let m = privkey.decrypt(&args.c).unwrap();
    println!("{}", m);
}
