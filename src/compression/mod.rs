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

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Compression {
    None = 0x00,
    Bzip = 0x10,
}

mod bzip;

/// Type alias for result of bytes
type ResultVector = ::std::io::Result<Vec<u8>>;

/// Compression trait
pub trait Compress {
    fn compress(&self, bytes: &Vec<u8>) -> ResultVector;
}

/// Deompression trait
pub trait Decompress {
    fn decompress(&self, bytes: &Vec<u8>) -> ResultVector;
}

/// Marker struct for bzip compression
pub struct BzipCompression;

/// Compress trait implementation
impl Compress for BzipCompression {
    /// Compress a vector of bytes with bzip2 [*PUBLIC*]
    fn compress(&self, bytes: &Vec<u8>) -> ResultVector {
        bzip::compress_bytes(&bytes)
    }
}

/// Deompress trait implementation
impl Decompress for BzipCompression {
    /// Decompress a vector of bytes with bzip2 [*PUBLIC*]
    fn decompress(&self, bytes: &Vec<u8>) -> ResultVector {
        bzip::decompress_bytes(&bytes)
    }
}
