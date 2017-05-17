# Bencoderus

Rust library for encoding and decoding the Bencode coding.

[![Build Status](https://travis-ci.org/russmack/bencoderus.svg?branch=master)](https://travis-ci.org/russmack/bencoderus)

## Usage
Decode:
```
// decode transforms Bencoded bytes to objects.
pub fn decode(src: Vec<u8>) -> Bencoding {
```
Encode:
```
// encode transforms objects to Bencoded bytes.
pub fn encode(benc: Bencoding) -> Vec<u8> {
```

## License
BSD 3-Clause: [LICENSE.txt](LICENSE.txt)

[<img alt="LICENSE" src="http://img.shields.io/pypi/l/Django.svg?style=flat-square"/>](LICENSE.txt)
