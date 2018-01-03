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
Useful in case you distrust the ability or willingness of your computer
to generate "sufficiently random" numbers. Even though `pgen` will
["do the right thing" and use `/dev/urandom`](https://sockpuppet.org/blog/2014/02/25/safely-generate-random-numbers/)
by default on Unix platforms, what if the hardware source(s) for the
entropy that the `/dev/urandom` CSPRNG is collecting is rigged?
With the `--dice` option you need not worry about *that* at least.
(But if you worry about that, have you considered the risk of [undetectable](http://www.tomsitpro.com/articles/it_security-rootkit-computer_security-computer_security,2-147-3.html) [malware](https://www.theregister.co.uk/2017/06/08/vxers_exploit_intels_amt_for_malwareoverlan/)?)

`-h`, `--help` Show help and exit.

## Installation

Build from source (see the [*Building*](#building) section below) and
copy the `target/release/pgen` binary into your `~/bin/` or whatever.

## Building

1. [Install Rust](https://www.rust-lang.org/en-US/install.html).
2. Issue `cargo build --release` in the root directory of the cloned repo.



[EFFWL]: https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases
