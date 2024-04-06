use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

// https://doc.rust-lang.org/cargo/reference/build-scripts.html#case-study-code-generation

/// Extract words from simple wordlist
fn words_simple(mut f_dest: &File, const_name: &str, fname_src: &str) {
    write!(f_dest, "pub const {const_name}: &[&str] = &[").unwrap();

    let f_src = BufReader::new(File::open(fname_src).unwrap());
    for line in f_src.lines() {
        match line {
            Ok(line) => {
                let word = line.trim();
                write!(f_dest, "\"{word}\",").unwrap();
            }
            Err(_e) => panic!("Unable to read line from internal file"),
        }
    }

    f_dest.write_all(b"];").unwrap();
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("wordlists.rs");
    let f = File::create(dest_path).unwrap();

    words_simple(&f, "WL_BIP39", "data/bip39_en_wordlist.txt");
}
