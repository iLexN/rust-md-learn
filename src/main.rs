fn main() {
    usage();
}

fn usage() {
    println!("Learning rust.");
    println!("Version: {}" , get_version());
}

fn get_version() -> u8 {
    let version = 1;
    if version < 2 {
        return 1;
    }

    2
}