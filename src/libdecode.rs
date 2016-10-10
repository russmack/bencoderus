//#![allow(dead_code)]
//#![allow(unused_variables)]

//pub use self::libdecode::*;

use std;
use std::collections::HashMap;
use std::str;

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
pub enum Bencoding {
    Integer(u64),
    ByteString(String),
    List(Vec<Bencoding>),
    Dictionary(HashMap<String, Bencoding>),
    Eof,
}

#[cfg(test)]
mod tests {
    use super::{
        Bencoding, 
        decode, 
        decode_byte_string_len, 
        extract_byte_string_length, 
        decode_number_unmarked};
    use std::collections::HashMap;

    struct TestCase {
        input: Vec<u8>,
        expected: Bencoding, 
    }

    #[test]
    fn test_decode_number() {
        let test_cases: Vec<TestCase> = vec![
            TestCase{ 
                input: vec![105, 49, 50, 51, 101], 
                expected: Bencoding::Integer(123) },
            TestCase{ 
                input: vec![105, 57, 56, 55, 101], 
                expected: Bencoding::Integer(987) },
        ];
        for t in test_cases {
            println!("test number decode...");
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
    fn test_decode_byte_string_len() {
        
        struct TestCase {
            input: Vec<u8>,
            max: u8,
            expected: Bencoding,
        }

        let test_cases: Vec<TestCase> = vec![
            TestCase{   
                input: vec![117, 118, 119, 120, 121, 122], 
                max: 5,
                expected: Bencoding::ByteString("uvwxy".to_string())
            },
            TestCase{
                input: vec![97, 98, 99, 100, 101, 102], 
                max: 4,
                expected: Bencoding::ByteString("abcd".to_string())
            },
        ];

        for t in test_cases {
            let mut iter = t.input.iter().peekable();
            let benc = decode_byte_string_len(&mut iter, t.max);

            let actual = match benc {
                Bencoding::ByteString(ref s) => s,
                _ => panic!("unexpected type"),
            };
            println!("expected: {:?}; got: {}", t.expected, actual);
            assert!(t.expected == benc);
        }
    }

    #[test]
    fn test_decode_number_unmarked() {
        
        let test_cases: Vec<TestCase> = vec![
            TestCase{   
                input: vec![51, 58, 117, 118, 119], 
                expected: Bencoding::Integer(3)
            },
            TestCase{   
                input: vec![54, 58, 117, 118, 119, 120, 121, 122], 
                expected: Bencoding::Integer(6)
            },
        ];

        for t in test_cases {
            let mut iter = t.input.iter().peekable();
            let benc = decode_number_unmarked(&mut iter, 58);
            let actual = match benc {
                Bencoding::Integer(ref n) => n,
                _ => panic!("unexpected type"),
            };
            println!("expected: {:?}; got: {}", t.expected, actual);
            assert!(t.expected == benc);
        }
    }

    #[test]
    fn test_extract_byte_string_length() {

        struct TestCase {
            input: Vec<u8>,
            expected: u64 
        }

        let test_cases: Vec<TestCase> = vec![
            TestCase{   
                input: vec![51, 58, 117, 118, 119], 
                expected: 3
            },
            TestCase{   
                input: vec![53, 58, 118, 119, 120, 121, 122], 
                expected: 5
            },
        ];

        for t in test_cases {
            let mut iter = t.input.iter().peekable();
            let actual = extract_byte_string_length(&mut iter); // , 58);
            /*
            let actual = match str_len {
                //Bencoding::ByteString(ref s) => s,
                n if ac => n,
                _ => panic!("unexpected type"),
            };
            */
            println!("expected: {:?}; got: {}", t.expected, actual);
            assert!(t.expected == actual);
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
            let benc = decode(t.input);
            let actual = match benc {
                Bencoding::ByteString(ref s) => s,
                _ => panic!("unexpected type"),
            };
            println!("expected: {:?}; got: {}", t.expected, actual);
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
            let benc = decode(t.input);
            match benc {
                Bencoding::ByteString(ref s) => s,
                _ => panic!("unexpected type"),
            };
        }
    }

    #[test]
    fn test_decode_list() {
        let test_cases: Vec<TestCase> = vec![
            TestCase{
                input: "l5:ItemA5:ItemBe".to_string().into_bytes(), // Vec<u8>
                expected:   Bencoding::List( vec![
                                    Bencoding::ByteString("ItemA".to_string()),
                                    Bencoding::ByteString("ItemB".to_string())
                                ])
            },
        ];

        for t in test_cases {
            let benc = decode(t.input);
            let actual = match benc {
                Bencoding::List(ref v) => v,
                _ => panic!("unexpected type"),
            };
            println!("expected: {:?}; got: {:?}", t.expected, actual);
            assert!(t.expected == benc);
        }
    }

    #[test]
    fn test_decode_dictionary() {
        let mut test_result: HashMap<String, Bencoding> = HashMap::new();
        test_result.insert("announce".to_string(), Bencoding::ByteString("http://192.168.1.74:6969/announce".to_string()));
        test_result.insert("comment".to_string(), Bencoding::ByteString("This is a comment".to_string()));
        let test_cases: Vec<TestCase> = vec![
            TestCase{
                input: "d8:announce33:http://192.168.1.74:6969/announce7:comment17:This is a commente".to_string().into_bytes(),
                expected: Bencoding::Dictionary(test_result)
            },
        ];

        for t in test_cases {
            let benc = decode(t.input);
            let actual = match benc {
                Bencoding::Dictionary(ref v) => v,
                _ => panic!("unexpected type"),
            };
            println!("expected: {:?}; got: {:?}", t.expected, actual);
            assert!(t.expected == benc);
        }
    }
}

pub fn decode(src: Vec<u8>) -> Bencoding {
    let mut iter_src = src.iter().peekable();
    decode_next(&mut iter_src)
}

pub fn iter_print<'a>(iter: &mut ::std::slice::Iter<'a, u32>) {
    let opt = iter.next();
    match opt {
        Some(v) => v,
        None => panic!("no val in iterator"),
    };
}

fn decode_next<'a>(mut iter: &mut std::iter::Peekable<std::slice::Iter<'a, u8>>) -> Bencoding {
    while let Some(&&curr) = iter.peek() {
        match curr {
            DICTIONARY_START => {
                //return Bencoding::Eof;
                return decode_dictionary(&mut iter);
            }
            LIST_START => {
                println!("decode_list()...");
                return decode_list(&mut iter);
            }
            NUMBER_START => {
                println!("decode_number()... starting with: {}", curr);
                return decode_number(&mut iter, NUMBER_END);
            }
            _ => {
                println!("decode_bytestring()... curr: {}", curr);
                return decode_byte_string(&mut iter);
            }
        }
    }
    Bencoding::Eof
}

fn decode_list<'a>(mut iter: &mut std::iter::Peekable<std::slice::Iter<'a, u8>>) -> Bencoding {
    // Skip over the list indicator character.
    iter.next();

    let mut list: Vec<Bencoding> = Vec::new();

    loop {
        {  // Scope so that we don't borrow iter as mutable more than once at a time.
            let opt = iter.peek();
            if let Some(&&v) = opt {
                if v == LIST_END {
                    break;
                }
            }
        }

        list.push(decode_next(&mut iter));
    }

    Bencoding::List(list)
}

fn  decode_dictionary<'a>(mut iter: &mut std::iter::Peekable<std::slice::Iter<'a, u8>>) -> Bencoding {
    // Skip over the dictionary indicator character.
    iter.next();

    let mut dict: HashMap<String, Bencoding> = HashMap::new();
    let mut keys: Vec<String> = Vec::new();
    
    loop {
        {
            let opt = iter.peek();
            if let Some(&&v) = opt {
               if v == DICTIONARY_END {
                    break;
                }
            }
        }
        let key = decode_byte_string(&mut iter);
        let val  = decode_next(&mut iter);
        
        let str_key = match key{
            Bencoding::ByteString(ref v) => v,
            _ => panic!("unexpected type"),
        };
        keys.push(str_key.to_owned());
        dict.insert(str_key.to_owned(), val);
    }
    
    // Verify that keys arrived sorted.
    let mut sorted_keys = keys.clone();
    sorted_keys.sort();
    for i in 0..keys.len()-1 {
        if keys[i] != sorted_keys[i] {
            panic!("dictionary keys not correctly sorted while decoding");
        }
    }
    Bencoding::Dictionary(dict)
}

// decode_number parses out a number token, discarding the initial start marker byte.
fn decode_number<'a>(iter: &mut std::iter::Peekable<std::slice::Iter<'a, u8>>, t: u8) -> Bencoding {
    let mut snum: String = String::new();
    let mut found_terminator: bool = false;
    while let Some(b) = iter.next() {
        match *b {
            NUMBER_START => {
                // Discard number start indicator.
                continue;
            }
            _ if *b == t => {
                // Handle terminator.
                found_terminator = true;
                break;
            }
            _ => {
                // Handle number characters.
                let barr = &[*b].to_owned();
                let bstr = str::from_utf8(barr);
                let s = match bstr.to_owned() {
                    Ok(v) => v,
                    Err(e) => panic!("error decoding number: {}", e),
                };
                snum.push_str(s);
            }
        };
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
fn extract_byte_string_length<'a>(mut iter: &mut std::iter::Peekable<std::slice::Iter<'a, u8>>) -> u64 {
    let benc_num = decode_number_unmarked(&mut iter, BYTE_ARRAY_DIVIDER);
    let len = match benc_num {
        Bencoding::Integer(v) => v,
        _ => panic!("unexpected type"),
    };
    len
}


fn decode_byte_string_len<'a>(iter: &mut std::iter::Peekable<std::slice::Iter<'a, u8>>, len: u8) -> Bencoding {
    let mut decr: u8 = len;
    let mut buf: Vec<u8> = Vec::new();
    while let Some(b) = iter.next() {
        buf.push(*b);
        decr = decr - 1;
        if decr == 0 {
            let a = String::from_utf8(buf);
            let s = match a {
                Ok(v) => v,
                Err(e) => panic!("error getting string: {}", e),
            };
            return Bencoding::ByteString(s);
        }
    }
    return Bencoding::ByteString("".to_string());
}


// decode_byte_string tokenises a byte string.
fn decode_byte_string<'a>(mut iter: &mut std::iter::Peekable<std::slice::Iter<'a, u8>>) -> Bencoding {
    let len = extract_byte_string_length(&mut iter); //, BYTE_ARRAY_DIVIDER);

    let mut buf: Vec<u8> = Vec::new();
    let mut i: u64 = 0;
    while let Some(b) = iter.next() {
        buf.push(*b);
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

// decode_number_unmarked does the same as decode_number_terminated
// but does not discard the current byte, instead including it in the result.
// unmarked refers to the number not being prefixed with i and suffixed with e.
fn decode_number_unmarked<'a>(iter: &mut std::iter::Peekable<std::slice::Iter<'a, u8>>, t: u8) -> Bencoding {
    let mut snum: String = String::new();
    let mut found_terminator: bool = false;

    while let Some(curr) = iter.next() {
        match *curr {
            _ if *curr == t  => {
                found_terminator = true;
                break;
            },
            _ => {
                // Handle number characters.
                let barr = &[*curr].to_owned();
                let bstr = str::from_utf8(barr);
                let s = match bstr.to_owned() {
                    Ok(v) => v,
                    Err(e) => panic!("error decoding number: {}", e),
                };
                snum.push_str(s);
            },
        }
    }

    if !found_terminator {
        while let Some(b) = iter.next() {
            if *b == t {
                found_terminator = true;
                break;
            }

            let f = &[*b].to_owned();
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
    }

    let p = snum.trim().parse::<u64>();
    let n = match p {
        Ok(v) => v,
        Err(e) => panic!("error parsing num: {}; err: {}", snum, e),
    };
    Bencoding::Integer(n)
}
