/*
 * Copyright (c) 2018, 2019, 2023 Erik Nordstrøm <erik@nordstroem.no>
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

use std::io;
use std::io::{stdin, stdout, Write};

use clap::crate_version;
use clap::load_yaml;
use clap::App;
use rand::thread_rng;
use rand::Rng;

// https://doc.rust-lang.org/cargo/reference/build-scripts.html#case-study-code-generation
include!(concat!(env!("OUT_DIR"), "/wordlists.rs"));

fn main() -> io::Result<()> {
    let yaml = load_yaml!("cli.yaml");
    let args = App::from_yaml(yaml).version(crate_version!()).get_matches();

    let opt_use_physical_dice = args.is_present("use_physical_dice");
    let opt_use_short_wlist = args.is_present("use_short_wlist");
    let opt_use_long_wlist = args.is_present("use_long_wlist");
    let opt_calculate_entropy = args.is_present("calc_entropy");

    let wordlist = {
        if opt_use_long_wlist {
            WL_LONG
        } else if opt_use_short_wlist {
            WL_SHORT
        } else {
            WL_AUTOCOMPLETE
        }
    };

    let num_dice: u32 = if opt_use_long_wlist { 5 } else { 4 };
    let wl_length = (6u32).pow(num_dice);

    let num_passphrases: u32 = {
        if args.is_present("num_passphrases") {
            args.value_of("num_passphrases")
                .unwrap()
                .parse::<u32>()
                .unwrap()
        } else {
            1
        }
    };

    let num_words: usize = {
        if args.is_present("num_words") {
            args.value_of("num_words")
                .unwrap()
                .parse::<usize>()
                .unwrap()
        } else if opt_use_long_wlist {
            10
        } else {
            12
        }
    };

    let stdout = stdout();
    let mut handle = stdout.lock();

    if opt_calculate_entropy {
        handle.write_fmt(format_args!(
            "Current settings will create passphrases with {:.2} bits of entropy.",
            (num_words as f64) * (wl_length as f64).log2()
        ))?;
    } else {
        for _ in 0..num_passphrases {
            if opt_use_physical_dice {
                let mut word_idx = vec![0usize; num_words];

                let width = format!("{num_words}").len();

                for i in 0..num_words {
                    eprint!("Word {:>w$} / {}. ", i + 1, num_words, w = width);
                    word_idx[i] = read_dice(num_dice);
                }

                for i in 0..num_words {
                    handle.write_all(wordlist[word_idx[i]].as_bytes())?;
                    if i < (num_words - 1) {
                        handle.write_all(b" ")?;
                    }
                }
            } else {
                let mut rng = thread_rng();

                for i in 0..num_words {
                    handle.write_all(wordlist[rng.gen_range(0, wl_length) as usize].as_bytes())?;
                    if i < (num_words - 1) {
                        handle.write_all(b" ")?;
                    }
                }
            }

            handle.write_all(b"\n")?;
        }
    }

    Ok(())
}

fn read_dice(n: u32) -> usize {
    eprint!("Throw {n} dice and enter the number of eyes shown on each: ");

    let mut result = 0;
    let mut i = 0;

    while i < n {
        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        for c in input.chars() {
            match c {
                '1' | '2' | '3' | '4' | '5' | '6' => {
                    result += (c.to_digit(10).unwrap() - 1) * (6u32).pow(n - i - 1);
                    i += 1;
                }
                _ => {}
            }

            if i == n {
                break;
            }
        }
    }

    result as usize
}
