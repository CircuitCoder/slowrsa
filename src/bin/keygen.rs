use slowrsa::key;

fn main() {
    let (privkey, pubkey) = slowrsa::key::keygen();
    println!("== PrivKey ==");
    privkey.print();
    println!("== PubKey ==");
    pubkey.print();
}
