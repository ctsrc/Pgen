use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

// https://doc.rust-lang.org/cargo/reference/build-scripts.html#case-study-code-generation

fn words(mut f_dest: &File, const_name: &str, fname_src: &str) {
    write!(f_dest, "const {const_name}: &[&str] = &[").unwrap();

    let f_src = BufReader::new(File::open(fname_src).unwrap());
    for line in f_src.lines() {
        match line {
            Ok(line) => {
                let word = line.split('\t').nth(1).unwrap();
                write!(f_dest, "\"{word}\",").unwrap();
            }
            Err(_e) => panic!("Unable to read line from internal file"),
        }
    }

    f_dest.write_all(b"];").unwrap();
}

fn words_simpler(mut f_dest: &File, const_name: &str, fname_src: &str) {
    write!(f_dest, "const {const_name}: &[&str] = &[").unwrap();

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

    words(&f, "WL_AUTOCOMPLETE", "eff_short_wordlist_2_0.txt");
    words(&f, "WL_LONG", "eff_large_wordlist.txt");
    words(&f, "WL_SHORT", "eff_short_wordlist_1.txt");
    words_simpler(&f, "WL_BIP39", "bip39_en_wordlist.txt");
}
