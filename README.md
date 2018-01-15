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
* [How many bits of entropy does your passphrase need?](#how-many-bits-of-entropy-does-your-passphrase-need)
* [Installation](#installation)

## Usage

```
pgen [--dice] [-l | -s] [-e] [-n <n>]
pgen -h | --help
pgen -V | --version
```

### Options

`-l` Use long wordlist instead of autocomplete-optimized short wordlist.
     Recommended for the creation of memorable passphrases since the
     increased number of words as well as the greater effective word
     length allows for good entropy with a lower amount of words
     compared to the autocomplete-optimized short wordlist.
     Mutually exclusive with option `-s`.

`-s` Use non-optimized short wordlist instead of autocomplete-optimized
     short wordlist. Mutually exclusive with option `-l`.

`-e` Print the entropy of the generated passphrase to stderr.
     What is password entropy? [Entropy is a measure of what the password
     could have been, so it relates to the selection process](https://crypto.stackexchange.com/a/376).

`-n` Specify the number of words to use *n*. Default value:

  * Twelve (12) words if either of the short wordlists are being used
    (meaning that the `-l` option was **not** specified).
  * Ten (10) words if the large wordlist is being used (meaning that
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

## How many bits of entropy does your passphrase need?

How many bits of entropy should your passphrase consist of?

Looking at [the article about password strength on Wikipedia](https://en.wikipedia.org/wiki/Password_strength), you will find that the following is said:

> The minimum number of bits of entropy needed for a password depends
> on the threat model for the given application. If
> [key stretching](https://en.wikipedia.org/wiki/Key_stretching)
> is not used, passwords with more entropy are needed.
> [RFC 4086](https://tools.ietf.org/html/rfc4086), "Randomness Requirements
> for Security", presents some example threat models and how to calculate
> the entropy desired for each one. Their answers vary between 29 bits
> of entropy needed if only online attacks are expected, and up to 128 bits
> of entropy needed for important cryptographic keys used in applications
> like encryption where the password or key needs to be secure for a long
> period of time and stretching isn't applicable.

In the case of web services such as webmail, social networks, etc.,
given that historically we have seen password databases leaked, where
weak hashing algorithms such as MD5 were used, it is the opinion of the
author that the neighbourhood of 128 bits of entropy is in fact
appropriate for such use.

When calculating the entropy of a password or a passphrase,
[one must assume that the password generation procedure is known to the attacker](https://crypto.stackexchange.com/a/376).
Hence with 12 words from either of the short wordlists, each of which
consist of 1296 words, we get a password entropy of log2(1296^12) ~=
124.08 bits. Similarily, with 10 words from the long wordlist (7776 words),
we get a password entropy of log2(7776^10) ~= 129.25 bits.

## Installation

1. [Install Rust](https://www.rust-lang.org/en-US/install.html).
2. Run `cargo install pgen`


[EFFWL]: https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases
