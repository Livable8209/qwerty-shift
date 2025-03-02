//! qwerty-shift
//! the basic idea is to shift each letter/number up the keyboard
//! for example, `z` become `a`, `a` becomes `q`, `q` becomes `1`
//! and `1` becomes `z`.
//!
//! jp[e moment: the program
//! started on 2025-03-01 at 3:15am lol
//! "finished" on 2025-03-01 at 8:53pm

use std::collections::HashMap;
use std::env;
use std::process::exit;

const _TEST_STR: &str = "the quick brown fox jumps over the lazy dog.";
const _TEST_ENCODED_STR: &str = "5y3 178di g492h r9s u7j0w 9f34 5y3 oqa6 e9tl";

fn main() {
    //this hashmap is awful to look at
    let encode_table = HashMap::from([
        //first column
        ('z', 'a'),
        ('a', 'q'),
        ('q', '1'),
        ('1', 'z'),
        ('!', 'Z'),
        //second column
        ('x', 's'),
        ('s', 'w'),
        ('w', '2'),
        ('2', 'x'),
        ('@', 'X'),
        //third column
        ('c', 'd'),
        ('d', 'e'),
        ('e', '3'),
        ('3', 'c'),
        ('#', 'C'),
        //forth column
        ('v', 'f'),
        ('f', 'r'),
        ('r', '4'),
        ('4', 'v'),
        ('$', 'V'),
        //fifth column
        ('b', 'g'),
        ('g', 't'),
        ('t', '5'),
        ('5', 'b'),
        ('%', 'B'),
        //sixth column
        ('n', 'h'),
        ('h', 'y'),
        ('y', '6'),
        ('6', 'n'),
        ('^', 'N'),
        //seventh column
        ('m', 'j'),
        ('j', 'u'),
        ('u', '7'),
        ('7', 'm'),
        ('&', 'M'),
        //eighth column
        (',', 'k'),
        ('k', 'i'),
        ('i', '8'),
        ('8', ','),
        ('*', '<'),
        ('<', 'K'),
        //ninth column
        ('.', 'l'),
        ('l', 'o'),
        ('o', '9'),
        ('9', '.'),
        ('(', '>'),
        //tenth column
        ('/', ';'),
        (';', 'p'),
        ('p', '0'),
        ('0', '/'),
        ('?', ':'),
        (':', 'P'),
        (')', '?'),
        //eleventh column
        ('\'', '['),
        ('[', '-'),
        ('-', '\''),
        ('"', '{'),
        ('{', '_'),
        ('_', '"'),
        //twelveth column
        (']', '='),
        ('=', ']'),
        ('}', '+'),
        ('+', '}'),
    ]);
    // Capital letters represent special cases

    //the decode_table code was ai generated, i probably could've figured it out myself but i got lazy :(
    let decode_table: HashMap<char, char> = encode_table
        .iter()
        .map(|(&decoded, &encoded)| (encoded, decoded))
        .collect();

    let mut sentence: Vec<String> = env::args().collect();

    if sentence.len() <= 1 {
        usage_msg();
    }

    //remove the file path from the sentence
    sentence.remove(0);

    let cipher_mode = match sentence.first() {
        Some(mode) => mode.to_string(),
        None => "encode".to_string(),
    };

    //remove the cipher mode to leave us with an untouched sentence
    sentence.remove(0);

    if cipher_mode == "decode" {
        println!("Decoding");
        let mut normal_sentence: Vec<String> = Vec::new();
        for word in &sentence {
            let mut normal_word = String::new();

            for letter in word.chars() {
                match decode_table.get(&letter) {
                    Some(shifted_letter) => {
                        normal_word.push(*shifted_letter);
                    }
                    None => println!("{letter} isnt a valid char"),
                }
            }

            normal_sentence.push(normal_word);
        }

        let out = normal_sentence.join(" ");
        println!("{out}");
    } else if cipher_mode == "encode" {
        println!("Encoding");
        let mut shifted_sentence: Vec<String> = Vec::new();
        for word in &sentence {
            let mut shifted_word = String::new();

            for letter in word.chars() {
                match encode_table.get(&letter) {
                    Some(shifted_letter) => {
                        shifted_word.push(*shifted_letter);
                    }
                    None => println!("{letter} isnt a valid char"),
                }
            }

            shifted_sentence.push(shifted_word);
        }

        let out = shifted_sentence.join(" ");
        println!("{out}");
    } else {
        usage_msg();
    }
}

fn usage_msg() {
    eprintln!("Usage: qwerty-shift [encode | decode] message");
    exit(0);
}
