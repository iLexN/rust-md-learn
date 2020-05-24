use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = get_input();

    match input.len() {
        2 => parse_markdown_file(&input[1]),
        //The default match case (_) will trigger if no other match case triggers
        _ => {
            println!("[ ERROR ] Invalid invocation (you done goofed!)");
            usage();
        }
    }
}

fn parse_markdown_file(_filename: &str) {
    print_short_banner();
    println!("[ INFO ] Trying to parse {}...", _filename);

    let input_filename = Path::new(_filename);
    let file = File::open(input_filename)
        //wrap the return value and pass along the Ok()–except upon failure,
        // in which case we output the string argument
        // to except as a kind of error message.
        .expect("[ ERROR ] Failed to open file!");

    // other way
    //let file = match File::open(&input_filename) {
    //   Err(err) => panic!("Couldn't open file: {}", err.description()),
    //   Ok(value) => value,
    // };

    let mut _ptag: bool = false; // keep track of paragraph tags
    let mut _htag: bool = false; // keep track of h1 tags

    let mut _token: Vec<String> = Vec::new();

    // Read the file line-by-line
    let reader = BufReader::new(file);

    for line in reader.lines() {
        // line is Result , so need check OK() , Err()
        // let line_content = match line {
        //     Ok(content) => content,
        //     Err(e) => panic!("Error: {}", e.to_string()),
        // };

        //also can manually unwrapping the Result object
        let line_content = line.unwrap();
        let mut _first_char: Vec<Char> = line_content
            //Get the line_contents variable and convert it to a sequence of characters.
            .chars().
            //Now take the first element of that iterable object.
            //Rust will convert this Iterator into a Take<char> object, a special kind of iterator
            take(1)
            //Now convert everything I have retrieved up to this point into a Collection–something
            // that I can subsequently use–of the type matching
            // the left-hand variable (which is Vec<char>)
            .collect();

    }
}

fn get_input() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();

    args
}

fn usage() {
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