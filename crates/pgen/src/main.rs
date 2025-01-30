/*
 * Copyright (c) 2018, 2019, 2023, 2024 Erik Nordstrøm <erik@nordstroem.no>
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

#![forbid(unsafe_code)]

use bip39_lexical_data::WL_BIP39;
use clap::{Parser, ValueEnum};
use eff_lexical_data::{WL_AUTOCOMPLETE, WL_LONG, WL_SHORT};
use rand::rng;
use rand::Rng;
use std::io::{stdin, stdout, Write};
use thiserror::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Use physical six-sided dice instead of letting the computer pick words
    #[arg(short = 'd', long = "dice")]
    use_physical_dice: bool,
    /// Select wordlist to use
    #[arg(short = 'w', long = "wordlist", value_enum, default_value_t)]
    use_wlist: WordlistChoice,
    /// Specify the number of passphrases to generate k
    #[arg(short, default_value_t = 1, value_name = "k")]
    k: u32,
    /// Specify the number of words to use
    #[arg(short, value_name = "n")]
    n: Option<usize>,
    /// Calculate and print the entropy for the passphrase(s) that would be generated with the given settings
    #[arg(short = 'e')]
    calculate_entropy: bool,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, ValueEnum)]
enum WordlistChoice {
    /// EFF's Short Wordlist #2
    ///
    /// Features:
    /// - Each word has a unique three-character prefix. This means that software could
    ///   auto-complete words in the passphrase after the user has typed the first three characters.
    /// - All words are at least an edit distance of 3 apart. This means that software could
    ///   correct any single typo in the user's passphrase (and in many cases more than one typo).
    ///
    /// Details:
    /// - <https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases>
    /// - <https://www.eff.org/dice>
    EffAutocomplete,
    /// EFF's Long Wordlist
    ///
    /// Features:
    /// - Contains words that are easy to type and remember.
    /// - Built from a list of words that prioritizes the most recognized words
    ///   and then the most concrete words.
    /// - Manually checked by EFF and attempted to remove as many profane, insulting, sensitive,
    ///   or emotionally-charged words as possible, and also filtered based on several public
    ///   lists of vulgar English words.
    ///
    /// Details:
    /// - <https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases>
    /// - <https://www.eff.org/dice>
    EffLong,
    /// EFF's Short Wordlist #1
    ///
    /// Features:
    /// - Designed to include the 1,296 most memorable and distinct words.
    ///
    /// Details:
    /// - <https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases>
    /// - <https://www.eff.org/dice>
    EffShort,
    /// BIP39 wordlist
    ///
    /// Details:
    /// - <https://en.bitcoin.it/wiki/BIP_0039>
    /// - <https://en.bitcoin.it/wiki/Seed_phrase>
    Bip39,
}

impl Default for WordlistChoice {
    fn default() -> Self {
        Self::EffAutocomplete
    }
}

#[derive(Debug, Error)]
enum Error {
    #[error("Invalid number of words for BIP39: {0}")]
    Bip39MSLenInvalid(usize),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let wordlist = match cli.use_wlist {
        WordlistChoice::EffAutocomplete => WL_AUTOCOMPLETE,
        WordlistChoice::EffLong => WL_LONG,
        WordlistChoice::EffShort => WL_SHORT,
        WordlistChoice::Bip39 => WL_BIP39,
    };

    // the EFF wordlists have lengths that are an exact power of 6,
    // whereas the bip39 wordlist does not
    let num_dice: u32 = if cli.use_wlist == WordlistChoice::Bip39 {
        // bip39 has 2048 words, which is a power of 2.
        // we need 11 dice because 6**11 / 3**11 = 2048,
        // i.e. we use 11 dice because it leads to a multiple
        // of the wordlist length
        11
    } else if cli.use_wlist == WordlistChoice::EffLong {
        // EFF long wordlist has 6**5 = 7776 words
        5
    } else {
        // Other EFF wordlists have 6**4 = 1296 words
        4
    };

    let num_passphrases = cli.k;

    let num_words = match cli.n {
        Some(n) => {
            // BIP39 has specific allowable lengths of the generated mnemonic sentence (MS) in words.
            // See <https://en.bitcoin.it/wiki/BIP_0039#Generating_the_mnemonic> for details.
            let bip39_allowable_mnemonic_sentence_lengths: [usize; 5] = [12, 15, 18, 21, 24];
            if cli.use_wlist == WordlistChoice::Bip39
                && !bip39_allowable_mnemonic_sentence_lengths.contains(&n)
            {
                eprintln!("When BIP39 wordlist is used, number of words to use must be one of: {bip39_allowable_mnemonic_sentence_lengths:?}");
                return Err(Error::Bip39MSLenInvalid(n).into());
            }
            n
        }
        None => {
            if cli.use_wlist == WordlistChoice::EffLong {
                10
            } else {
                12
            }
        }
    };

    let stdout = stdout();
    let mut handle = stdout.lock();

    if cli.calculate_entropy {
        handle.write_fmt(format_args!(
            "Current settings will create passphrases with {:.2} bits of entropy.\n",
            (num_words as f64) * (wordlist.len() as f64).log2()
        ))?;
    } else {
        for _ in 0..num_passphrases {
            if cli.use_physical_dice {
                let mut word_idx = vec![0usize; num_words];

                let width = format!("{num_words}").len();

                for (i, item) in word_idx.iter_mut().enumerate().take(num_words) {
                    eprint!("Word {:>w$} / {}. ", i + 1, num_words, w = width);
                    // For the sake of the bip39 wordlist, we modulo index by the wordlist length,
                    // because the range of the possible values is a multiple of the wordlist length.
                    //
                    // With the EFF wordlists, the wordlist lengths match the range
                    // of the numbers we get from the dice, so for EFF wordlists
                    // this modulo does not change anything.
                    *item = read_dice(num_dice) % wordlist.len();
                }

                for i in 0..num_words {
                    handle.write_all(wordlist[word_idx[i]].as_bytes())?;
                    if i < (num_words - 1) {
                        handle.write_all(b" ")?;
                    }
                }
            } else {
                let mut rng = rng();

                for i in 0..num_words {
                    handle.write_all(
                        wordlist[rng.random_range(0..wordlist.len()) as usize].as_bytes(),
                    )?;
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
