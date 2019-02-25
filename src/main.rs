use clap::{App, Arg};
use std::io::{self, BufRead, Write};

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
                if c.is_ascii_alphabetic() {
                    last_upcase = false;
                }
            } else {
                let prev = c;
                c.make_ascii_uppercase();
                if prev != c {
                    last_upcase = true;
                }
            }
            c
        })
        .collect()
}

fn annoyify_stdin(random: bool) {
    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();
    let mut buf = String::new();

    loop {
        stdin_handle.read_line(&mut buf).expect("Reading from stdin failed!");
        if buf.is_empty() {
            break;
        }

        let annoyified = if random {
            annoyify_random(&buf)
        } else {
            annoyify_alternating(&buf)
        };

        stdout_handle.write(annoyified.as_bytes()).expect("Writing to stdout failed!");
        buf.clear();
    }
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

    if input == "-" {
        annoyify_stdin(matches.is_present("random"));
    } else {
        if matches.is_present("random") {
            println!("{}", annoyify_random(&input));
        } else {
            println!("{}", annoyify_alternating(&input));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        assert_eq!(annoyify_alternating("this is a test"), String::from("ThIs Is A tEsT"));
    }

    #[test]
    fn ignore_punctuation() {
        assert_eq!(annoyify_alternating("this. is. a. test."), String::from("ThIs. Is. A. tEsT."));
    }
}
