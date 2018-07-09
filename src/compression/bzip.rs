// flatarchive
// Copyright (C) 2018 Samantha Enders
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use bzip2::{read::BzDecoder, write::BzEncoder, Compression};

use std::io;
use std::io::prelude::*;

/// Compress with bzip2
/// _WARNING_ Internal method, likely to break between versions
/// Use bzip::BzipCompression instead
pub fn decompress_bytes(bytes: &Vec<u8>) -> io::Result<Vec<u8>> {
    let mut decompressor = BzDecoder::new(bytes.as_slice());
    let mut decompressed = Vec::new();
    match decompressor.read_to_end(&mut decompressed) {
        Ok(_) => Ok(decompressed),
        Err(err) => Err(err),
    }
}

/// Compress with bzip2
/// _WARNING_ Internal method, likely to break between versions
/// Use bzip::BzipCompression instead
pub fn compress_bytes(bytes: &Vec<u8>) -> io::Result<Vec<u8>> {
    let buffer = Vec::new();
    let mut compressor = BzEncoder::new(buffer, Compression::Best);
    if let Err(err) = compressor.write_all(&bytes) {
        return Err(err);
    }
    compressor.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress() {
        use bzip2::write::BzEncoder;
        ::quickcheck::quickcheck(test as fn(_) -> _);

        fn test(v: Vec<u8>) -> bool {
            let mut e = BzEncoder::new(Vec::new(), Compression::Best);
            if let Err(err) = e.write_all(&v) {
                println!("err encoding {:?}", err);
                return false;
            }
            let buffer = e.finish().unwrap();
            let result = decompress_bytes(&buffer).unwrap();
            result == v
        }
    }

    #[test]
    fn test_compress_empty() {
        ::quickcheck::quickcheck(test as fn(_) -> _);

        fn test(v: Vec<u8>) -> bool {
            let result = compress_bytes(&v);
            let result = result.unwrap();
            let mut reader = BzDecoder::new(result.as_slice());
            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer).unwrap();
            buffer == v
        }
    }
}
