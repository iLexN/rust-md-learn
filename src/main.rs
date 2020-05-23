fn main() {
    usage();
}

fn usage() {
    println!("Learning rust.");

    let the_version = env!("CARGO_PKG_VERSION");
    println!("Version: {}", the_version);
}

// fn get_version() -> u8 {
//     let version = 1;
//     if version < 2 {
//         return 1;
//     }
//
//     2
// }