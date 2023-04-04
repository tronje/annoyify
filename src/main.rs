use clap::{App, Arg};
use std::io::{self, BufRead, Write};

fn annoyify_random(bytes: &mut [u8]) {
    for byte in bytes.iter_mut() {
        if rand::random() {
            byte.make_ascii_uppercase();
        } else {
            byte.make_ascii_lowercase();
        }
    }
}

fn annoyify_alternating(bytes: &mut [u8]) {
    let mut last_upcase = false;

    for byte in bytes.iter_mut() {
        if last_upcase && byte.is_ascii_alphabetic() {
            last_upcase = false;
            byte.make_ascii_lowercase();
        } else if byte.is_ascii_alphabetic() {
            byte.make_ascii_uppercase();
            last_upcase = true;
        }
    }
}

fn annoyify_stdin(random: bool) -> io::Result<()> {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    loop {
        let length = {
            let mut buf = stdin.fill_buf()?.to_owned();
            let len = buf.len();
            if len == 0 {
                break;
            }

            if random {
                annoyify_random(&mut buf);
            } else {
                annoyify_alternating(&mut buf);
            }

            stdout.write_all(&buf)?;

            len
        };

        stdin.consume(length);
    }

    Ok(())
}

fn annoyify_phrase(input: &str, random: bool) -> io::Result<()> {
    let mut stdout = io::stdout().lock();

    let mut buf = input.to_owned().into_bytes();

    if random {
        annoyify_random(&mut buf);
        stdout.write_all(&buf)?;
    } else {
        annoyify_alternating(&mut buf);
        stdout.write_all(&buf)?;
    }

    // write an extra newline
    stdout.write_all(b"\n")?;

    Ok(())
}

fn main() -> io::Result<()> {
    let matches = App::new("annoyify")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Annoyify a phrase")
        .arg(
            Arg::with_name("INPUT")
                .help("The input string to annoyify. If not present, stdin will be annoyified."),
        )
        .arg(
            Arg::with_name("random")
                .short("r")
                .long("random")
                .help("Randomize instead of alternating"),
        )
        .get_matches();

    let input = matches.value_of("INPUT");
    let random = matches.is_present("random");

    if let Some(phrase) = input {
        annoyify_phrase(phrase, random)
    } else {
        annoyify_stdin(random)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let mut before = "this is a test".to_owned().into_bytes();
        let expected = "ThIs Is A tEsT".to_owned().into_bytes();
        annoyify_alternating(&mut before);
        assert_eq!(before, expected);
    }

    #[test]
    fn ignore_punctuation() {
        let mut before = "this. is. a. test.".to_owned().into_bytes();
        let expected = "ThIs. Is. A. tEsT.".to_owned().into_bytes();
        annoyify_alternating(&mut before);
        assert_eq!(before, expected);
    }
}
