fn main() {
    usage();
}

fn usage() {
    print_short_banner()
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn get_title() -> String {
    let mut the_title = String::from("");
    the_title.push_str(env!("CARGO_PKG_NAME"));
    the_title.push_str(" (v");
    the_title.push_str(env!("CARGO_PKG_VERSION"));
    the_title.push_str("),");
    the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));

    the_title
}

// fn get_version() -> u8 {
//     let version = 1;
//     if version < 2 {
//         return 1;
//     }
//
//     2
// }