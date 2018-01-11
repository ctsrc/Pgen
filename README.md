# pgen – Passphrase Generator

[![xkcd: Password Strength](https://imgs.xkcd.com/comics/password_strength.png)](https://xkcd.com/936/)

Generate passphrases using the [wordlists for random passphrases][EFFWL]
made by the EFF.

By default, generated passphrases consist of twelve words randomly
selected from the autocomplete-optimized wordlist. Be sure to
[read the article][EFFWL] to learn about the difference between the
wordlists.

## Table of Contents

* [Usage](#usage)
  - [Options](#options)
* [Installation](#installation)
* [Building](#building)

## Usage

```
pgen [--dice] [-l | -s] [-e] [-n <n>]
pgen -h | --help
```

### Options

`-l` Use long wordlist instead of autocomplete-optimized short wordlist.
     Recommended for the creation of memorable passphrases since the
     increased word length allows for good entropy with a lower amount
     of words compared to the autocomplete-optimized short wordlist.
     Mutually exclusive with option `-s`.

`-s` Use non-optimized short wordlist instead of autocomplete-optimized
     short wordlist. Mutually exclusive with option `-l`.

`-e` Print the entropy of the generated passphrase to stderr.

`-n` Specify the number of words to use *n*. Default value:

  * Twelve (12) words if the autocomplete-optimized wordlist is being used
    (meaning that neither the `-s` nor the `-l` option was specified).
  * Eight (8) words if the non-optimized short wordlist is being used
    (meaning that the `-s` option was specified).
  * Six (6) words if the large wordlist is being used (meaning that
    the `-l` option was specified.)

`--dice` Use physical six-sided dice instead of letting the computer pick
words. Useful in case you distrust the ability or willingness of your
computer to generate "sufficiently random" numbers. Even though `pgen` will
[*do the right thing* and use `/dev/urandom`](https://sockpuppet.org/blog/2014/02/25/safely-generate-random-numbers/)
by default on Unix platforms \[[1](https://doc.rust-lang.org/rand/rand/index.html)\],
what if the hardware source(s) for the entropy that the `/dev/urandom`
CSPRNG is collecting is/are rigged? With the `--dice` option
you need not worry about *that* at least. (But have you considered
the risk of *undetectable malware*? \[[2](http://www.tomsitpro.com/articles/it_security-rootkit-computer_security-computer_security,2-147-3.html)\], \[[3](https://www.theregister.co.uk/2017/06/08/vxers_exploit_intels_amt_for_malwareoverlan/)\])

`-h`, `--help` Show help and exit.

`-V`, `--version` Print version information and exit.

## Installation

Build from source (see the [*Building*](#building) section below) and
copy the `target/release/pgen` binary into your `~/bin/` or whatever.

## Building

1. [Install Rust](https://www.rust-lang.org/en-US/install.html).
2. Issue `cargo build --release` in the root directory of the cloned repo.



[EFFWL]: https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases
