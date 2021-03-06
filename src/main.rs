use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

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

    let mut tokens: Vec<String> = Vec::new();

    // not same as https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
    // why???
    // Read the file line-by-line
    let reader = BufReader::new(file);

    for line in reader.lines() {
        // line is Result , so need check OK() , Err()
        // let line_content = match line {
        //     Ok(content) => content,
        //     Err(e) => panic!("Error: {}", e.to_string()),
        // };

        //also can manually unwrapping the Result object
        let line_contents = line.unwrap();
        let mut first_char: Vec<char> = line_contents
            //Get the line_contents variable and convert it to a sequence of characters.
            .chars().
            //Now take the first element of that iterable object.
            //Rust will convert this Iterator into a Take<char> object, a special kind of iterator
            take(1)
            //Now convert everything I have retrieved up to this point into a Collection–something
            // that I can subsequently use–of the type matching
            // the left-hand variable (which is Vec<char>)
            .collect();

        let mut output_line = String::new();

        match first_char.pop() {
            Some('#') => put_header(&line_contents, &mut output_line),
            _ => put_paragraph(&line_contents, &mut output_line)
        };

        if output_line != "<p></p>\n".to_string() {
            tokens.push(output_line);
        }
    }

    let output_filename = get_output_filename(&_filename);
    println!("file name: {}", output_filename);

    write_to_html(&mut tokens, output_filename);

    println!("[ INFO ] Parsing complete!");
}

fn get_output_filename(_filename: &str) -> String {
    let mut output_filename = String::from(&_filename[.._filename.len() - 3]);
    output_filename.push_str(".html");
    output_filename
}

fn write_to_html(tokens: &mut Vec<String>, output_filename: String) {
    let mut outfile = File::create(output_filename)
        .expect("[ ERROR ] Could not create output file!");

    //Remember that we borrow a reference to the tokens vector (like this: &tokens)
    //because of Rust’s ownership rules.
    // If we didn’t include that &,
    // the value of each element in tokens would be moved into the for-loop
    // and removed from outside of it–and we don’t want that!
    for line in tokens {
        outfile.write_all(line.as_bytes())
            .expect("[ ERROR ] Could not write to output file!");
    }
}

fn put_paragraph(line_contents: &String, output_line: &mut String) {
    output_line.push_str("<p>");
    output_line.push_str(&line_contents);
    output_line.push_str("</p>\n");
}

fn put_header(line_contents: &String, output_line: &mut String) {
    output_line.push_str("<h1>");
    output_line.push_str(&line_contents[2..]); // Get all but the first two characters
    output_line.push_str("</h1>\n");
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