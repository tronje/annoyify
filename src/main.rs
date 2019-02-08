use clap::{App, Arg};
use std::io::{self, Read};

fn annoyify_random(s: &str) -> String {
    let out = s.to_lowercase();

    out.chars()
        .map(|mut c| {
            if rand::random() {
                c.make_ascii_uppercase();
            }
            c
        })
        .collect()
}

fn annoyify_alternating(s: &str) -> String {
    let out = s.to_lowercase();
    let mut last_upcase = false;

    out.chars()
        .map(|mut c| {
            if last_upcase {
                last_upcase = false;
            } else {
                c.make_ascii_uppercase();
                last_upcase = true;
            }
            c
        })
        .collect()
}

fn main() {
    let matches = App::new("annoyify")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Annoyify a phrase")
        .arg(Arg::with_name("INPUT").help("The input string to annoyify, or `-' for stdin"))
        .arg(
            Arg::with_name("random")
                .short("r")
                .long("random")
                .help("Randomize instead of alternating"),
        )
        .get_matches();

    let input = matches
        .value_of("INPUT")
        .unwrap_or("you need to provide an input");

    let mut buffer = String::new();

    if input == "-" {
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("Reading from stdin failed!");
    } else {
        buffer += input;
    }

    if matches.is_present("random") {
        println!("{}", annoyify_random(&buffer));
    } else {
        println!("{}", annoyify_alternating(&buffer));
    }
}
