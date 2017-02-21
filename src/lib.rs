#![crate_name = "bencoderus"]
#![crate_type = "lib"]

pub mod libdecode;
pub mod libencode;

const ASCII_D: u8 = 100;
const ASCII_E: u8 = 101;
const ASCII_I: u8 = 105;
const ASCII_L: u8 = 108;
const ASCII_COLON: u8 = 58;

pub const DICTIONARY_START: u8 = ASCII_D;
pub const DICTIONARY_END: u8 = ASCII_E;
pub const LIST_START: u8 = ASCII_L;
pub const LIST_END: u8 = ASCII_E;
pub const NUMBER_START: u8 = ASCII_I;
pub const NUMBER_END: u8 = ASCII_E;
pub const BYTE_ARRAY_DIVIDER: u8 = ASCII_COLON;

use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
pub enum Bencoding {
    Integer(u64),
    ByteString(String),
    List(Vec<Bencoding>),
    Dictionary(HashMap<String, Bencoding>),
    Eof,
}
