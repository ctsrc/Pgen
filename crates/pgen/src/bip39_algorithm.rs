/*
 * Copyright (c) 2024 Erik Nordstrøm <erik@nordstroem.no>
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
use sha2::{Digest, Sha256};

/// Calculate BIP39 checksum (CS) bits given entropy bits.
fn calculate_cs_bits(ent: &[u8]) -> u8 {
    let mut hasher = Sha256::new();
    hasher.update(ent);
    let hash = hasher.finalize();
    let shift = match ent.len() {
        // 128 bits of entropy (16 bytes) needs 4 bits of checksum
        16 => 4usize,
        // 160 bits of entropy (20 bytes) needs 5 bits of checksum
        20 => 3,
        // 192 bits of entropy (24 bytes) needs 6 bits of checksum
        24 => 2,
        // 224 bits of entropy (28 bytes) needs 7 bits of checksum
        28 => 1,
        // 256 bits of entropy (32 bytes) needs 8 bits of checksum
        32 => 0,
        // No other number of bits of entropy aside from the above is supported by BIP39.
        // And since this function is internal to our program, and we only intend to call it
        // with the supported number of bits of entropy, there really isn't much point in going
        // through the extra motions of returning an error since it would mean we have a fatal
        // (unrecoverable) error in the coding of our program anyway. So we may as well panic
        // via `unreachable!()` instead of returning details about the error.
        _ => unreachable!(),
    };
    hash[0] >> shift
}

/// Get BIP39 English word from 11 bits.
fn get_word_from_11_bits(value: u16) -> &'static str {
    // The caller is responsible for ensuring that only the lower 11 bits are set.
    const MAX_ACCEPTABLE_VALUE: u16 = 0b11111111111;
    if value > MAX_ACCEPTABLE_VALUE {
        unreachable!();
    }
    WL_BIP39[value as usize]
}

/// Extract 11 bit chunks from entropy bytes.
///
/// Returns a `Vec<u16>` of 11 bit chunks, along with an `usize` specifying
/// the number of bits that are left over for checksum in the last `u16` element of the `Vec`.
fn chunk_to_11_bit_groups(ent: &[u8]) -> (Vec<u16>, usize) {
    let mut chunks = vec![];

    // Initialize first output chunk. Initially empty.
    let mut curr_output_chunk = 0u16;
    // Number of bits left for curr chunk to be complete
    let mut cc = 11;

    for &curr_input_byte in ent.iter() {
        eprintln!("curr_input_byte      {curr_input_byte:#010b}");

        // Number of bits left unused in curr input byte
        let mut iu = 8;

        // Take all bits from input byte, filling output chunks.
        let mut left_over = curr_input_byte;
        while iu != 0 {
            eprintln!("left over            {left_over:#010b}");
            let take_n_bits = if cc >= iu { iu } else { cc };
            cc -= take_n_bits;
            eprintln!("cc                           {cc:#2}");
            iu -= take_n_bits;
            eprintln!("iu                           {cc:#2}");

            let mask = 0xffu8 >> (8 - take_n_bits);

            curr_output_chunk += ((curr_input_byte & mask) as u16) << cc;
            eprintln!("curr_output_chunk {curr_output_chunk:#013b}");

            if cc == 0 {
                chunks.push(curr_output_chunk);
                curr_output_chunk = 0;
                cc = 11;
            }
            if iu != 0 {
                left_over = curr_input_byte >> (8 - iu);
            }
        }
        eprintln!();
    }
    if cc != 11 {
        chunks.push(curr_output_chunk);
    } else {
        cc = 0;
    }

    for chunk in &chunks {
        eprintln!("chunk             {chunk:#013b}");
    }

    (chunks, cc)
}

#[cfg(test)]
mod test {
    use crate::bip39_algorithm::{
        calculate_cs_bits, chunk_to_11_bit_groups, get_word_from_11_bits,
    };
    use test_case::test_case;

    // From <https://github.com/trezor/python-mnemonic/blob/b57a5ad77a981e743f4167ab2f7927a55c1e82a8/vectors.json#L3-L8>:
    //
    // ```json
    // [
    //     "00000000000000000000000000000000",
    //     "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
    //     "c55257c360c07c72029aebc1b53c05ed0362ada38ead3e3e9efa3708e53495531f09a6987599d18264c1e1c92f2cf141630c7a3c4ab7c81b2f001698e7463b04",
    //     "xprv9s21ZrQH143K3h3fDYiay8mocZ3afhfULfb5GX8kCBdno77K4HiA15Tg23wpbeF1pLfs1c5SPmYHrEpTuuRhxMwvKDwqdKiGJS9XFKzUsAF"
    // ],
    // ```
    //
    // - 128 bits of "entropy" (all zero in this case).
    // - The 12th word in the mnemonic sentence is the 4th word (index 3) in the BIP39 English wordlist.
    #[test_case(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], 3; "with 128 bits of input of all zeros")]
    // From <https://github.com/trezor/python-mnemonic/blob/b57a5ad77a981e743f4167ab2f7927a55c1e82a8/vectors.json#L27-L32>:
    //
    // ```json
    // [
    //     "000000000000000000000000000000000000000000000000",
    //     "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon agent",
    //     "035895f2f481b1b0f01fcf8c289c794660b289981a78f8106447707fdd9666ca06da5a9a565181599b79f53b844d8a71dd9f439c52a3d7b3e8a79c906ac845fa",
    //     "xprv9s21ZrQH143K3mEDrypcZ2usWqFgzKB6jBBx9B6GfC7fu26X6hPRzVjzkqkPvDqp6g5eypdk6cyhGnBngbjeHTe4LsuLG1cCmKJka5SMkmU"
    // ],
    // ```
    //
    // - 192 bits of "entropy" (all zero in this case).
    // - The 18th word in the mnemonic sentence is the 40th word (index 39) in the BIP39 English wordlist.
    #[test_case(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], 39; "with 192 bits of input of all zeros")]
    // From <https://github.com/trezor/python-mnemonic/blob/b57a5ad77a981e743f4167ab2f7927a55c1e82a8/vectors.json#L51-L56>:
    //
    // ```json
    // [
    //     "0000000000000000000000000000000000000000000000000000000000000000",
    //     "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art",
    //     "bda85446c68413707090a52022edd26a1c9462295029f2e60cd7c4f2bbd3097170af7a4d73245cafa9c3cca8d561a7c3de6f5d4a10be8ed2a5e608d68f92fcc8",
    //     "xprv9s21ZrQH143K32qBagUJAMU2LsHg3ka7jqMcV98Y7gVeVyNStwYS3U7yVVoDZ4btbRNf4h6ibWpY22iRmXq35qgLs79f312g2kj5539ebPM"
    // ],
    // ```
    //
    // - 256 bits of "entropy" (all zero in this case).
    // - The 24th word in the mnemonic sentence is the 103rd word (index 102) in the BIP39 English wordlist.
    #[test_case(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], 102; "with 256 bits of input of all zeros")]
    fn calculates_cs_bits_correctly(ent: &[u8], cs_expected: u8) {
        let cs_actual = calculate_cs_bits(ent);
        assert_eq!(cs_expected, cs_actual);
    }

    #[test_case(0, "abandon"; "first word in list (index 0)")]
    #[test_case(3, "about")]
    #[test_case(102, "art")]
    #[test_case(2047, "zoo"; "last word in list (index 2047)")]
    fn gets_correct_word_from_11_bits(value: u16, expected_word: &str) {
        let actual_word = get_word_from_11_bits(value);
        assert_eq!(expected_word, actual_word);
    }

    #[test]
    #[should_panic]
    fn get_word_should_panic_when_more_than_11_bits_are_set() {
        let value = 2048u16;
        let _ = get_word_from_11_bits(value);
    }

    #[test_case(&[0xff, 0xff], &[0b11111111111, 0b11111000000], 6; "simple non-BIP39 input")]
    // 128 bits of input should have 12 chunks of output, with 4 bits left in last byte for checksum, according to BIP39.
    #[test_case(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], &[0,0,0,0,0,0,0,0,0,0,0,0], 4; "with 128 bits of input of all zeros")]
    #[test_case(&[0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff], &[0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0b11111110000], 4; "with 128 bits of input of all ones")]
    #[test_case(&[0xff,0,0xff,0,0xff,0,0xff,0,0xff,0,0xff,0,0xff,0,0xff,0], &[2040,63,1537,2032,127,1027,2016,255,7,1984,510,0], 4; "with 128 bits of input alternating between bytes all one and all zero")]
    #[test_case(&[0,0xff,0,0xff,0,0xff,0,0xff,0,0xff,0,0xff,0,0xff,0,0xff], &[7,1984,510,15,1920,1020,31,1792,2040,63,1537,2032], 4; "with 128 bits of input alternating between bytes all zero and all one")]
    // 160 bits of input should have 15 chunks of output, with 5 bits left in last byte for checksum, according to BIP39.
    #[test_case(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], &[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], 5; "with 160 bits of input of all zeros")]
    #[test_case(&[0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff], &[0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0b11111100000], 5; "with 160 bits of input of all ones")]
    // 192 bits of input should have 18 chunks of output, with 6 bits left in last byte for checksum, according to BIP39.
    #[test_case(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], &[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], 6; "with 192 bits of input of all zeros")]
    #[test_case(&[0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff], &[0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0b11111000000], 6; "with 192 bits of input of all ones")]
    // 224 bits of input should have 21 chunks of output, with 7 bits left in last byte for checksum, according to BIP39.
    #[test_case(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], &[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], 7; "with 224 bits of input of all zeros")]
    #[test_case(&[0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff], &[0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0b11110000000], 7; "with 224 bits of input of all ones")]
    // 256 bits of input should have 24 chunks of output, with 8 bits left in last byte for checksum, according to BIP39.
    #[test_case(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], &[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], 8; "with 256 bits of input of all zeros")]
    #[test_case(&[0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff], &[0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0x7ff,0b11100000000], 8; "with 256 bits of input of all ones")]
    fn chunks_correctly_to_11_bit_groups(
        input_ent: &[u8],
        expected_chunks: &[u16],
        expected_n_cs: usize,
    ) {
        let (actual_chunks, actual_n_cs) = chunk_to_11_bit_groups(input_ent);
        // The output chunks should be as we think they should be.
        assert_eq!(expected_chunks, actual_chunks);
        // The number of lower bits left for checksum in the last output chunk should be as we think it should.
        assert_eq!(expected_n_cs, actual_n_cs);
        // Only the lower 11 bits should be set in each output chunk.
        for actual_chunk in actual_chunks {
            assert_eq!(actual_chunk, actual_chunk & 0b11111111111);
        }
    }
}
