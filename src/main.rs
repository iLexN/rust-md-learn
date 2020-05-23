fn main() {
    usage();
}

fn usage() {
    println!("Learning rust.");

    let version: u8;
    version = get_version();
    println!("Version: {}", version);
}

fn get_version() -> u8 {
    let version = 1;
    if version < 2 {
        return 1;
    }

    2
}