use super::*;

use std::str;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use Bencoding;
    use libdecode::{
        decode,
        decode_byte_string_len, 
        extract_byte_string_length, 
        decode_number_unmarked};

    struct TestCase {
        input: Vec<u8>,
        expected: Bencoding, 
    }

    #[test]
    fn test_decode() {
        // d8:completei1e10:downloadedi0e10:incompletei0e8:intervali1924e12:min intervali962e5:peers6:V(ݓe
        let mut test_result: HashMap<Vec<u8>, Bencoding> = HashMap::new();
        test_result.insert(
            "complete".to_string().into_bytes(), 
            Bencoding::Integer(1));
        test_result.insert(
            "downloaded".to_string().into_bytes(), 
            Bencoding::Integer(0));
        test_result.insert(
            "incomplete".to_string().into_bytes(), 
            Bencoding::Integer(0));
        test_result.insert(
            "interval".to_string().into_bytes(), 
            Bencoding::Integer(1924));
        test_result.insert(
            "min interval".to_string().into_bytes(), 
            Bencoding::Integer(962));
        test_result.insert(
            "peers".to_string().into_bytes(), 
            Bencoding::ByteString(vec![31, 55, 16, 128, 221, 147]));

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                //d8:completei1e10:downloadedi0e10:incompletei0e8:intervali1924e12:min intervali962e5:peers6:V(ݓe

                input: vec![100, 56, 58, 99, 111, 109, 112, 108, 101, 116, 101, 105, 49, 101, 49, 48, 58, 100, 111, 119, 110, 108, 111, 97, 100, 101, 100, 105, 48, 101, 49, 48, 58, 105, 110, 99, 111, 109, 112, 108, 101, 116, 101, 105, 48, 101, 56, 58, 105, 110, 116, 101, 114, 118, 97, 108, 105, 49, 57, 50, 52, 101, 49, 50, 58, 109, 105, 110, 32, 105, 110, 116, 101, 114, 118, 97, 108, 105, 57, 54, 50, 101, 53, 58, 112, 101, 101, 114, 115, 54, 58, 31, 55, 16, 128, 221, 147, 101],
                expected: Bencoding::Dictionary(test_result),
            },
        ];

        struct DictPair {
            key: Vec<u8>, 
            val: Bencoding,
        }

        for t in test_cases {
            println!("test decode...");
            let benc = decode(t.input);
            println!("exp: {:?}", t.expected);
            println!("got: {:?}", benc);

            let mut pair_list_expect: Vec<DictPair> = vec![];
            let mut pair_list_actual: Vec<DictPair> = vec![];

            let dict_expect = match t.expected {
                Bencoding::Dictionary(ref v) => v,
                _ => panic!("fix your test"),
            };
            let dict_actual = match benc {
                Bencoding::Dictionary(ref v) => v,
                _ => panic!("fix your test"),
            };

            for (k, v) in dict_expect.iter() {
                pair_list_expect.push(DictPair{key: k.to_vec(), val: v.clone()});
            }
            for (k, v) in dict_actual.iter() {
                pair_list_actual.push(DictPair{key: k.to_vec(), val: v.clone()});
            }

            pair_list_expect.sort_by(|a, b| a.key.cmp(&b.key));
            pair_list_actual.sort_by(|a, b| a.key.cmp(&b.key));

            for i in 0..pair_list_actual.len() {
                println!("testing key a == e: {:?}  ==  {:?}", pair_list_actual[i].key, pair_list_expect[i].key);
                println!("testing val a == e: {:?}  ==  {:?}", pair_list_actual[i].val, pair_list_expect[i].val);
                assert_eq!(pair_list_actual[i].key, pair_list_expect[i].key);
                assert_eq!(pair_list_actual[i].val, pair_list_expect[i].val);
            }
        }
    }

    #[test]
    fn test_decode_peers_compacted() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                // d8:completei1e5:peers6:V(ݓe
                input: vec![100, 56, 58, 99, 111, 109, 112, 108, 101, 116, 101, 105, 49, 101, 53, 58, 112, 101, 101, 114, 115, 54, 58, 86, 40, 221, 147, 101],
                expected: Bencoding::ByteString("V(ݓe".as_bytes().to_vec()),
            },
        ];
        for t in test_cases {
            println!("test decode peers compacted...");
            let benc = decode(t.input);
            let d = match benc {
                Bencoding::Dictionary(ref d) => d,
                _ => panic!("test expected Bencoding::Dictionary"),
            };
            let dp = &d["peers".as_bytes()];
            assert!(t.expected == *dp);
        }
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
                expected: Bencoding::ByteString("uvwxy".to_string().into_bytes())
            },
            TestCase{
                input: vec![97, 98, 99, 100, 101, 102], 
                max: 4,
                expected: Bencoding::ByteString("abcd".to_string().into_bytes())
            },
        ];

        for t in test_cases {
            let mut iter = t.input.iter().peekable();
            let benc = decode_byte_string_len(&mut iter, t.max);

            let actual = match benc {
                Bencoding::ByteString(ref s) => s,
                _ => panic!("unexpected type"),
            };
            println!("expected: {:?}; got: {:?}", t.expected, actual);
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
            let actual = extract_byte_string_length(&mut iter);
            println!("expected: {:?}; got: {}", t.expected, actual);
            assert!(t.expected == actual);
        }
    }

    #[test]
    fn test_decode_byte_string() {
        let test_cases: Vec<TestCase> = vec![
            TestCase{   
                input: vec![51, 58, 117, 118, 119], 
                expected: Bencoding::ByteString("uvw".to_string().into_bytes())
            },
            TestCase{   
                input: vec![51, 58, 120, 121, 122], 
                expected: Bencoding::ByteString("xyz".to_string().into_bytes())
            },
        ];

        for t in test_cases {
            let benc = decode(t.input);
            let actual = match benc {
                Bencoding::ByteString(ref s) => s,
                _ => panic!("unexpected type"),
            };
            println!("expected: {:?}; got: {:?}", t.expected, actual);
            assert!(t.expected == benc);
        }
    }

    #[test]
    #[should_panic]
    fn failing_test_decode_byte_string() {
        let test_cases: Vec<TestCase> = vec![
            TestCase{
                input: vec![65, 67, 69], 
                expected: Bencoding::ByteString("ACE".to_string().into_bytes())
            },
            TestCase{
                input: vec![69, 67, 65], 
                expected: Bencoding::ByteString("ECA".to_string().into_bytes())
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
                input: "l5:ItemA5:ItemBe".to_string().into_bytes(), 
                expected:   Bencoding::List( vec![
                                    Bencoding::ByteString("ItemA".to_string().into_bytes()),
                                    Bencoding::ByteString("ItemB".to_string().into_bytes())
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
        let mut test_result: HashMap<Vec<u8>, Bencoding> = HashMap::new();
        test_result.insert(
            "announce".to_string().into_bytes(), 
            Bencoding::ByteString("http://192.168.1.74:6969/announce".to_string().into_bytes()));
        test_result.insert(
            "comment".to_string().into_bytes(), 
            Bencoding::ByteString("This is a comment".to_string().into_bytes()));
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

// decode transforms Bencoded bytes to objects.
pub fn decode(src: Vec<u8>) -> Bencoding {
    let mut iter_src = src.iter().peekable();
    decode_next(&mut iter_src)
}

pub fn iter_print<'a>(iter: &mut std::slice::Iter<'a, u32>) {
    let opt = iter.next();
    match opt {
        Some(v) => v,
        None => panic!("no val in iterator"),
    };
}

fn decode_next<'a>(mut iter: &mut std::iter::Peekable<std::slice::Iter<'a, u8>>) -> Bencoding {
    if let Some(&&curr) = iter.peek() {
        match curr {
            DICTIONARY_START => {
                return decode_dictionary(&mut iter);
            }
            LIST_START => {
                return decode_list(&mut iter);
            }
            NUMBER_START => {
                return decode_number(&mut iter, NUMBER_END);
            }
            _ => {
                return decode_byte_string(&mut iter);
            }
        }
    }
    Bencoding::Eof
}

// decode_list reads the text source representing a list and constructs
// and returns a list object - a vec.
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

fn decode_dictionary<'a>(mut iter: &mut std::iter::Peekable<std::slice::Iter<'a, u8>>) -> Bencoding {
    // Skip over the dictionary indicator character.
    iter.next();

    let mut dict: HashMap<Vec<u8>, Bencoding> = HashMap::new();
    let mut keys: Vec<Vec<u8>> = Vec::new();

    loop {
        {
            let opt = iter.peek();
            if opt == None {
                break;
            }

            if let Some(&&v) = opt {
               if v == DICTIONARY_END {
                    break;
                }
            }
        }
        let key = decode_byte_string(&mut iter);
        let val = decode_next(&mut iter);
        
        let str_key = match key{
            Bencoding::ByteString(ref v) => v,
            _ => panic!("unexpected type"),
        };

        let key_vec = str_key.to_vec();
        let key_vec_2 = key_vec.clone();
        keys.push(key_vec);
        dict.insert(key_vec_2, val);
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

    for b in iter {
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
    match benc_num {
        Bencoding::Integer(v) => v,
        _ => panic!("unexpected type"),
    }
}


#[allow(dead_code)]
fn decode_byte_string_len<'a>(iter: &mut std::iter::Peekable<std::slice::Iter<'a, u8>>, len: u8) -> Bencoding {
    let mut decr: u8 = len;
    let mut buf: Vec<u8> = Vec::new();
    for b in iter {
        buf.push(*b);
        decr -= 1;
        if decr == 0 {
            return Bencoding::ByteString(buf);
        }
    }
    Bencoding::ByteString(buf)
}


// decode_byte_string tokenises a byte string.
fn decode_byte_string<'a>(mut iter: &mut std::iter::Peekable<std::slice::Iter<'a, u8>>) -> Bencoding {
    let len = extract_byte_string_length(&mut iter);

    let mut buf: Vec<u8> = Vec::new();
    if len == 0 {
        return Bencoding::ByteString(buf);
    }

    for (mut i, b) in iter.enumerate() {
        buf.push(*b);
        i += 1;
        if i as u64 == len {
            break;
        }
    }
    Bencoding::ByteString(buf)
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
                    Err(e) => panic!("error decoding unmarked number: {}", e),
                };
                snum.push_str(s);
            },
        }
    }

    if !found_terminator {
        for b in iter {
            if *b == t {
                found_terminator = true;
                break;
            }

            let f = &[*b].to_owned();
            let g = str::from_utf8(f);
            let d = match g.to_owned() {
                Ok(v) => v,
                Err(e) => panic!("error decoding unmarked number: {}", e),
            };
            snum.push_str(d);
        }

        if !found_terminator {
            panic!("error, unmarked number not terminated".to_string());
        }
    }

    let p = snum.trim().parse::<u64>();
    let n = match p {
        Ok(v) => v,
        Err(e) => panic!("error parsing num: {}; err: {}", snum, e),
    };
    Bencoding::Integer(n)
}

