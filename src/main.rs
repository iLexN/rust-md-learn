fn main() {
    let input = get_input();
    println!("arg {}" , input[0]);

    match input.len() {
        2 => do_something(),
        //The default match case (_) will trigger if no other match case triggers
        _ => {
            println!("[ ERROR ] Invalid invocation (you done goofed!)");
            usage();
        }
    }
}

fn do_something(){

}

fn get_input() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();

    args
}

fn usage() {
    print_short_banner();
    print_long_banner();
}

fn print_long_banner() {
    println!("{}", get_title());
    println!("Written by: {}", env!("CARGO_PKG_AUTHORS"));
    println!("Homepage: {}", env!("CARGO_PKG_HOMEPAGE"));
    println!("Usage: tinymd <somefile>.md");

    //v2
    //print_short_banner();
    // let mut written_by = String::from("Written by: ");
    // written_by.push_str(env!("CARGO_PKG_AUTHORS"));
    // let mut homepage = String::from("Homepage: ");
    // homepage.push_str(env!("CARGO_PKG_HOMEPAGE"));
    // let mut usage = String::from("Usage: tinymd <somefile>.md");
    // println!("{}", written_by);
    // println!("{}", homepage);
    // println!("{}", usage);

    //v3
    // print_short_banner();
    // println!("Written by: {}\nHomepage: {}\nUsage: tinymd <somefile>.md\n",
    //          env!("CARGO_PKG_AUTHORS"),
    //          env!("CARGO_PKG_HOMEPAGE")
    // );
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