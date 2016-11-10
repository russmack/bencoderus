#![allow(dead_code)]
#![allow(unused_variables)]


pub use super::*;

mod tests {

    use super::{Bencoding, run, decode, encode};

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
        for t in test_cases {
            let decode_input = t.input.clone();
            let test_input = t.input.clone();
            let benc = decode(decode_input);
            let str = encode(benc);
            assert!(test_input == str);   
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
        _ => panic!("unexepected type"),
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

fn encode_list(mem_stream: &mut Vec<u8>, benc_list: Bencoding) {
    mem_stream.push(LIST_START);
    let mut val = match benc_list {

    }
}

fn encode_dictionary(mem_stream: &mut Vec<u8>, benc_dict: Bencoding) {

}

