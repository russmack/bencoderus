#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

pub use super::*;

mod tests {

    use super::{Bencoding, run, decode, encode};
    use std::collections::HashMap;

    #[test]
    fn test_run() {
        run();
        assert!(true);
    }

    struct TestCase {
        input: Vec<u8>,
        expected: Bencoding,
    }

    #[test]
    fn test_encode_number() {
        let test_cases: Vec<TestCase> = vec![
            TestCase{
                input: vec![105, 49, 50, 51, 101],
                expected: Bencoding::Integer(123) },
            TestCase{
                input: vec![105, 57, 56, 55, 101],
                expected: Bencoding::Integer(987) },
        ];

        // Note that for the tests we're both decoding and encoding.
        // Partly just because it was easier.
        for t in test_cases {
            println!("test number encode...");
            let decode_input = t.input.clone();
            let test_input = t.input.clone();
            let benc = decode(decode_input);
            let num = encode(benc);
            println!("expect: {:?} ; got: {:?}", test_input, num);
            assert!(test_input == num);
        }
    }

    #[test]
    fn test_encode_bytestring() {
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

        // Note that for the tests we're both decoding and encoding.
        // Partly just because it was easier.
        for t in test_cases {
            let decode_input = t.input.clone();
            let test_input = t.input.clone();
            let benc = decode(decode_input);
            let str = encode(benc);
            assert!(test_input == str);   
        }
    }

    #[test]
    fn test_encode_list() {
        let test_cases: Vec<TestCase> = vec![
            TestCase{
                input: "l5:ItemA5:ItemBe".to_string().into_bytes(), // Vec<u8>
                expected: Bencoding::List( vec![
                                    Bencoding::ByteString("ItemA".to_string()),
                                    Bencoding::ByteString("ItemB".to_string())
                                ])
            },
        ];

        // Note that for the tests we're both decoding and encoding.
        // Partly just because it was easier.
        for t in test_cases {
            let decode_input = t.input.clone();
            let test_input = t.input.clone();
            let benc = decode(decode_input);
            let str = encode(benc);
            assert!(test_input == str);
        }
    }

    #[test]
    fn test_encode_dictionary() {

        let mut test_result: HashMap<String, Bencoding> = HashMap::new();
        test_result.insert(
            "announce".to_string(), 
            Bencoding::ByteString("http://192.168.1.74:6969/announce".to_string()));
        test_result.insert(
            "comment".to_string(), 
            Bencoding::ByteString("This is a comment".to_string()));
        let test_cases: Vec<TestCase> = vec![
            TestCase{
                input: "d8:announce33:http://192.168.1.74:6969/announce7:comment17:This is a commente".to_string().into_bytes(),
                expected: Bencoding::Dictionary(test_result)
            },
        ];

        // Note that for the tests we're both decoding and encoding.
        // Partly just because it was easier.
        for t in test_cases {
            let decode_input = t.input.clone();
            let test_input = t.input.clone();
            let benc = decode(decode_input);
            let s = encode(benc);
            assert!(test_input == s);
        }

    }
}

fn run() {
    // let b = Bencoding::Integer(4);
}

fn encode(benc: Bencoding) -> Vec<u8> {
    let mut mem_stream = vec![];
    encode_next(&mut mem_stream, benc);
    mem_stream
}

fn encode_next(mem_stream: &mut Vec<u8>, obj: Bencoding) {
    // let t: Bencoding = match obj {
    match obj {
        Bencoding::Integer(ref v) => {
            //println!("integer found");
            encode_number(mem_stream, Bencoding::Integer(*v))
        }
        Bencoding::ByteString(ref v) => {
            //println!("byte string found")
            let v_2 = v.clone();
            encode_bytestring(mem_stream, Bencoding::ByteString(v_2))
        }
        Bencoding::List(ref v) => {
            let v_2 = v.clone();
            encode_list(mem_stream, Bencoding::List(v_2))
        }
        Bencoding::Dictionary(ref v) => {
            let v_2 = v.clone();
            encode_dictionary(mem_stream, Bencoding::Dictionary(v_2))
        }
        _ => {
            println!("panic on obj: {:?}", obj);
            panic!("unexepected type");
        }
    };
}

fn encode_number(mem_stream: &mut Vec<u8>, num: Bencoding) {
    mem_stream.push(NUMBER_START);
    let mut val = match num {
        Bencoding::Integer(ref v) => v.to_string().into_bytes(),
        _ => panic!("unexpected type"),
    };
    mem_stream.append(&mut val);
    mem_stream.push(NUMBER_END);
}

fn encode_bytestring(mem_stream: &mut Vec<u8>, benc_str: Bencoding) {
    let mut str = match benc_str {
        Bencoding::ByteString(ref v) => v.to_string().into_bytes(),
        _ => panic!("unexpected type"),
    };
    mem_stream.append(& mut str.len().to_string().into_bytes());
    mem_stream.push(58);
    mem_stream.append(& mut str);
}

// enocde_list converts a list (a vec) into bencoding formatted bytes and
// writes them to the mem_stream, a mut vec<u8>.
fn encode_list(mem_stream: &mut Vec<u8>, benc_list: Bencoding) {
    mem_stream.push(LIST_START);
    // input: "l5:ItemA5:ItemBe".to_string().into_bytes(), // Vec<u8>
    let mut val = match benc_list {
        Bencoding::List(ref v) => v, 
        _ => panic!("unexpected type"),
    };
    for i in val {
        let ii = i.clone();
        encode_next(mem_stream, ii);
    }
    mem_stream.push(LIST_END);
}

fn encode_dictionary(mem_stream: &mut Vec<u8>, benc_dict: Bencoding) {
    mem_stream.push(DICTIONARY_START);
    // input: d3:bar4:spam3:fooi42ee
    let mut val = match benc_dict {
        Bencoding::Dictionary(ref v) => v,
        _ => panic!("unexpected type"),
    };

    let mut keys: Vec<String> = vec![];
    for key in val.keys() {
        let key_2 = key.clone();
        keys.push(key_2);
    }
    keys.sort();
    for k in keys {
        let kk = k.clone();

        let vv = match val.get(&kk) {
            Some(o) => o.clone(),
            _ => panic!("no such key in dictionary"),
        };

        encode_next(mem_stream, Bencoding::ByteString(kk));
        encode_next(mem_stream, vv);
    }
    mem_stream.push(DICTIONARY_END);
}
