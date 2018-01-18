/*
 * Copyright (c) 2018 Erik Nordstrøm <erik@nordstroem.no>
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

use std::io::stdin;

#[macro_use]
extern crate clap;
use clap::App;

extern crate rand;
use rand::os::OsRng;
use rand::Rng;

// https://doc.rust-lang.org/cargo/reference/build-scripts.html#case-study-code-generation
include!(concat!(env!("OUT_DIR"), "/wordlists.rs"));

fn read_dice (n: u32) -> usize
{
    eprint!("Throw {} dice and enter the number of eyes shown on each: ", n);

    let mut result = 0;
    let mut i = 0;

    while i < n
    {
        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        for c in input.chars()
        {
            match c
            {
                '1' | '2' | '3' | '4' | '5' | '6' =>
                {
                    result += (c.to_digit(10).unwrap() - 1) *
                        (6 as u32).pow(n - i - 1);
                    i += 1;
                },
                _ => {}
            }

            if i == n
            {
                break;
            }
        }
    }

    return result as usize;
}

fn main ()
{
    let yaml = load_yaml!("cli.yaml");
    let args = App::from_yaml(yaml).get_matches();

    let opt_use_physical_dice = args.is_present("use_physical_dice");
    let opt_use_short_wlist = args.is_present("use_short_wlist");
    let opt_use_long_wlist = args.is_present("use_long_wlist");

    let num_dice: u32 = if opt_use_long_wlist { 5 } else { 4 };
    let wl_length = (6 as u32).pow(num_dice);

    let num_words: usize =

        if args.is_present("num_words")
        {
            args.value_of("num_words").unwrap().parse::<usize>().unwrap()
        }
        else if opt_use_long_wlist
        {
            10
        }
        else
        {
            12
        };

    let wordlist =

        if opt_use_long_wlist
        {
            WL_LONG
        }
        else if opt_use_short_wlist
        {
            WL_SHORT
        }
        else
        {
            WL_AUTOCOMPLETE
        };

    if opt_use_physical_dice
    {
        let mut word_idx = vec![0 as usize; num_words];

        for i in 0..num_words
        {
            eprint!("Word {}. ", i + 1);
            word_idx[i] = read_dice(num_dice);
        }

        for i in 0..num_words
        {
            print!("{}", wordlist[word_idx[i]]);

            if i < (num_words - 1)
            {
                print!(" ");
            }
        }
    }
    else
    {
        let mut rng = OsRng::new().unwrap();

        for i in 0..num_words
        {
            print!("{}", wordlist[rng.gen_range(0, wl_length) as usize]);

            if i < (num_words - 1)
            {
                print!(" ");
            }
        }
    }

    println!();
}
