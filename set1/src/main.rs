use clap::{Arg, Command};
use hex;
use std::fs::File;
use std::io::prelude::*;

pub mod eight;
pub mod five;
pub mod four;
pub mod one;
pub mod seven;
pub mod six;
pub mod three;
pub mod two;

fn main() {
    let cli = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .subcommand(Command::new("three").about("break single-byte xor"))
        .subcommand(
            Command::new("five")
                .about("repeating-key xor")
                .arg(
                    Arg::new("file")
                        .help("file to encrypt/decrypt")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("key")
                        .help("the encryption/decryption key")
                        .required(true)
                        .index(2),
                ),
        )
        .arg_required_else_help(true)
        .get_matches();

    if let Some(_) = cli.subcommand_matches("three") {
        let freqmap: [f64; 256] = three::initialize_frequency_map();
        let ciphertext: Vec<u8> =
            hex::decode(three::STRING_3).expect("error hex-decoding plaintext");
        let plaintexts = three::break_single_byte_xor(ciphertext, &freqmap)
            .expect("error breaking single byte xor");
        for pt in plaintexts {
            println!("{}", std::str::from_utf8(&pt).unwrap());
        }
    } else if let Some(matches) = cli.subcommand_matches("five") {
        let filename: String = matches.get_one::<String>("file").unwrap().to_string();
        let key: String = matches.get_one::<String>("key").unwrap().to_string();

        //get contents of file as a string
        let mut file = File::open(filename).expect("error opening file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("error reading file");

        //pass two repeating_key_xor fn with key
        let xored_file =
            five::repeating_key_xor(&contents.as_bytes().to_vec(), key.as_bytes().to_vec());

        println!("{}", hex::encode(&xored_file));
    }
}
