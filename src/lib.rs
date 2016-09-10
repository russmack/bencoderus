#![allow(dead_code)]
#![allow(unused_variables)]

//#[macro_use] extern crate log;

use std::str;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;


const ASCII_D: u8 = 100;
const ASCII_E: u8 = 101;
const ASCII_I: u8 = 105;
const ASCII_L: u8 = 108;
const ASCII_COLON: u8 = 58;

const DICTIONARY_START: u8 = ASCII_D; //String::from("d").into_bytes()[0];
const DICTIONARY_END: u8 = ASCII_E;
const LIST_START: u8 = ASCII_L;
const LIST_END: u8 = ASCII_E;
const NUMBER_START: u8 = ASCII_I;
const NUMBER_END: u8 = ASCII_E;
const BYTE_ARRAY_DIVIDER: u8 = ASCII_COLON;

#[derive(PartialEq, Clone, Debug)]
//#[derive(PartialEq, Copy, Clone, Debug)]
enum Bencoding {
    Integer(u64),
    ByteString(String),
    //ByteString(&'static str),
    List,
    Dictionary,
    Eof,
}

#[cfg(test)]
mod tests {
    use super::{Bencoding, decode};

    struct TestCase {
        input: Vec<u8>,
        expected: Bencoding, /* expected: Result<Bencoding, String>,
                              * expected: Result<Bencoding, &'static Error>, // expected: Result<Bencoding, String>, */
    }

    #[test]
    fn test_decode_number() {
        let test_cases: Vec<TestCase> = vec![
            TestCase{ input: vec![105, 49, 50, 51, 101] , expected: Bencoding::Integer(123) },
            TestCase{ input: vec![105, 57, 56, 55, 101], expected: Bencoding::Integer(987) },

        ];
        for t in test_cases {
            println!("test decode...");
            let benc = decode(t.input);
            assert!(t.expected == benc);
        }
    }

    #[test]
    #[should_panic]
    fn failing_test_decode_number() {
        struct FailTestCase {
            input: Vec<u8>,
        }

        let fail_test_cases: Vec<FailTestCase> = vec![
            FailTestCase{ input: vec![105, 57, 56, 55]},
        ];
        for t in fail_test_cases {
            decode(t.input);
        }
    }

    #[test]
    fn test_decode_byte_string() {
        let test_cases: Vec<TestCase> = vec![
            TestCase{   
                input: vec![51, 58, 117, 118, 119], 
                expected: Bencoding::ByteString("uvw".to_string())
            },
            TestCase{   
                input: vec![51, 58, 120, 121, 122], 
                expected: Bencoding::ByteString("xyz".to_string())
            },
        ];

        for t in test_cases {
            let benc  = decode(t.input);
            let actual = match benc {
                Bencoding::ByteString(ref s) => s,
                _ => panic!("unexpected type"),
            };
            println!("expected: {:?}; got: {}", t.expected, actual );
            assert!(t.expected == benc);
        }
    }

    #[test]
    #[should_panic]
    fn failing_test_decode_byte_string() {
        let test_cases: Vec<TestCase> = vec![
            TestCase{
                input: vec![65, 67, 69], 
                expected: Bencoding::ByteString("ACE".to_string())
            },
            TestCase{
                input: vec![69, 67, 65], 
                expected: Bencoding::ByteString("ECA".to_string())
            },
        ];

        for t in test_cases {
            let benc  = decode(t.input);
            let actual = match benc {
                Bencoding::ByteString(ref s) => s,
                _ => panic!("unexpected type"),
            };
            //println!("expected: {:?}; got: {}", t.expected, actual );
            //assert!(t.expected == benc);
        }
    }

}

fn decode(src: Vec<u8>) -> Bencoding {
    println!("decoding...");
    let (tx, rx): (Sender<u8>, Receiver<u8>) = mpsc::channel();
    println!("channels created...");
    let h = thread::spawn(|| scan(tx, src));
    println!("spawned scan...");

    decode_next(&rx)
}

fn scan(tx: Sender<u8>, src: Vec<u8>) {
    //println!("scanning...");
    for i in src {
        let res = tx.send(i);
        match res {
            Ok(_) => (),
            Err(e) => panic!("error sending on channel: {}", e),
        };
    }
}

fn decode_next(rx: &Receiver<u8>) -> Bencoding {
    //debug!("logging... decoding next...");
    println!("decoding next...");
    for curr in rx {
        match curr {
            /*
            DICTIONARY_START => {
                return Bencoding::Eof;
                // return decode_dictionary(&rx);
            }
            LIST_START => {
                return Bencoding::Eof;
                // return decode_list(&rx);
            }
            */
            NUMBER_START => {
                println!("decoding number...");
                return decode_number(&rx, curr, NUMBER_END) ;
            }
            _ => {
                println!("decoding bytestring...");
                //return Bencoding::Eof;
                return decode_byte_string(&rx, curr);
            }
        }
    }
    Bencoding::Eof
}

// decode_number parses out a number token, discarding the initial start marker byte.
fn decode_number(rx: &Receiver<u8>, curr: u8, t: u8) -> Bencoding {
    let mut snum: String = String::new();

    let mut found_terminator: bool = false;
    for b in rx {
        if b == t {
            found_terminator = true;
            break;
        }

        let barr = &[b].to_owned();
        let bstr = str::from_utf8(barr);
        let s = match bstr.to_owned() {
            Ok(v) => v,
            Err(e) => panic!("error decoding number: {}", e),
        };
        snum.push_str(s);
    }
    if !found_terminator {
        panic!("error, number not terminated".to_string());
    }

    let n = match snum.trim().parse::<u64>() {
        Ok(v) => v, 
        Err(e) => panic!("error parsing number: {}; err: {}", snum, e),
    };
    Bencoding::Integer(n)
}

// extract_byte_string_length returns the length of a byte string.
// The length of the byte string prefixes the byte string, with a delimiter.
fn extract_byte_string_length(rx: &Receiver<u8>, curr: u8, delim: u8) -> u64 {
    let benc_num = decode_number_unmarked(rx, curr, BYTE_ARRAY_DIVIDER);
    let len = match benc_num {
        Bencoding::Integer(v) => v,
        _ => panic!("unexpected type"),
    };
    len
}

// decode_byte_string tokenises a byte string.
fn decode_byte_string(rx: &Receiver<u8>, curr: u8) -> Bencoding {
    let len = extract_byte_string_length(rx, curr, BYTE_ARRAY_DIVIDER);    

    let mut buf: Vec<u8> = Vec::new();
    let mut i: u64 = 0;
    for b in rx {
        buf.push(b);
        i = i + 1;
        if i == len {
            break;
        }
    }

    // NOTE !!!
    // pieces: A 20 character SHA1 hash is generated for each piece.
    // These are joined together into one large byte array (byte[][]).
    // BUT!!! : from_utf8() fails, maybe length is twice the number of chars
    // to handle two-byte chars, and from_utf8() can't handle these chars -
    // find a suitable from_.... func.

    let a = String::from_utf8(buf);
    let s = match a {
        Ok(v) => v,
        Err(e) => panic!("error getting string: {}", e),
    };
    
    return Bencoding::ByteString(s.to_string());
}

// decode_number_terminated_with_peek does the same as decode_number_terminated
// but does not discard the current byte, instead including it in the result.
fn decode_number_unmarked(rx: &Receiver<u8>, curr: u8, t: u8) -> Bencoding {
    let mut snum: String = String::new();

    println!("curr: {}", curr);
    let f = &[curr].to_owned();
    let g = str::from_utf8(f);
    let d = match g.to_owned() {
        Ok(v) => v,
        Err(e) => panic!("error decoding number: {}", e),
    };
    snum.push_str(d);

    let mut found_terminator: bool = false;
    for b in rx {
        println!("decoding... {}", b);
        if b == t {
            found_terminator = true;
            break;
        }
        println!("decoding... should have terminated {}", b);

        let f = &[b].to_owned();
        let g = str::from_utf8(f);
        let d = match g.to_owned() {
            Ok(v) => v,
            Err(e) => panic!("error decoding number: {}", e),
        };
        snum.push_str(d);
    }

    if !found_terminator {
        panic!("error, number not terminated".to_string());
    }

    let p = snum.trim().parse::<u64>();
    let n = match p {
        Ok(v) => v,
        Err(e) => panic!("error parsing num: {}; err: {}", snum, e),
    };
    //return Box::new(n);
    Bencoding::Integer(n)
}
































