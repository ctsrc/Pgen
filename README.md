# pgen -- Passphrase Generator

[![xkcd: Password Strength](https://imgs.xkcd.com/comics/password_strength.png)](https://xkcd.com/936/)

Generate passphrases using the [wordlists for random passphrases][EFFWL]
made by the EFF.

By default, generated passphrases consist of six words randomly selected
from the large wordlist.

Be sure to [read the article about the wordlists][EFFWL] to learn about
the difference between them.

## Table of Contents

* [Usage](#usage)
  - [Options](#options)
* [Installation](#installation)
* [Building](#building)

## Usage

```
pgen [-s | -a] [-n *n*] [-e] [--dice]
pgen -h | --help
```

### Options

`-s` Use short wordlist. Mutually exclusive with option `-a`.

`-a` Use autocomplete-optimized wordlist.
Mutually exclusive with option `-s`.

`-n` Specify the number of words to use *n*. Default value:

  * Six (6) words if the large wordlist is being used (meaning that
    neither the `-s` nor the `-a` option was specified).
  * Eight (8) words if the short wordlist is being used (meaning that
    the `-s` option was specified).
  * Twelve (12) words if the autocomplete-optimized wordlist is being
    used (meaning that the `-a` option was specified).

`-e` Print the entropy of the generated passphrase to stderr.

`--dice` Use six-sided dies instead of letting the computer pick words.
Useful in case you distrust the ability of your computer to generate
"sufficiently random" numbers.

`-h`, `--help` Show help and exit.

## Installation

Build from source (see the [*Building*](#building) section below) and
copy the `target/release/pgen` binary into your `~/bin/` or whatever.

## Building

1. [Install Rust](https://www.rust-lang.org/en-US/install.html).
2. Issue `cargo build --release` in the root directory of the cloned repo.



[EFFWL]: https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases
