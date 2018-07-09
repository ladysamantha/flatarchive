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

use compression::Compression;

pub struct Entry {
    pub offset: usize,
    pub is_dir: bool,
    pub name: String,
    pub compression: Compression,
    pub bytes: Vec<u8>,
}

pub struct EntryBuilder {
    entry: Entry,
}

impl Entry {
    pub fn empty() -> Entry {
        Entry {
            offset: 0,
            is_dir: false,
            name: String::new(),
            compression: Compression::None,
            bytes: Vec::new(),
        }
    }
}

impl EntryBuilder {
    pub fn new() -> EntryBuilder {
        EntryBuilder {
            entry: Entry::empty(),
        }
    }

    pub fn with_name(mut self, name: &str) -> EntryBuilder {
        self.entry.name = name.into();
        self
    }

    pub fn with_offset(mut self, offset: usize) -> EntryBuilder {
        self.entry.offset = offset;
        self
    }

    pub fn with_is_dir(mut self, is_dir: bool) -> EntryBuilder {
        self.entry.is_dir = is_dir;
        self
    }

    pub fn with_compression(mut self, compression: &Compression) -> EntryBuilder {
        self.entry.compression = *compression;
        self
    }

    pub fn build(self) -> Entry {
        self.entry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_creates_entry() {
        let entry: Entry = EntryBuilder::new().build();
        assert_eq!(entry.offset, 0);
        assert_eq!(entry.is_dir, false);
        assert_eq!(entry.name, "");
        assert_eq!(entry.compression, Compression::None);
        assert_eq!(entry.bytes, Vec::new());
    }

    #[test]
    fn builder_name() {
        let entry: Entry = EntryBuilder::new().with_name("foo").build();

        assert_eq!(entry.name, "foo");
    }

    #[test]
    fn builder_offset() {
        let offset: usize = 0x001010;
        let entry: Entry = EntryBuilder::new().with_offset(offset as usize).build();

        assert_eq!(entry.offset, offset);
    }

    #[test]
    fn builder_is_dir() {
        let is_dir = true;
        let entry: Entry = EntryBuilder::new().with_is_dir(is_dir).build();

        assert_eq!(entry.is_dir, is_dir);
    }

    #[test]
    fn builder_compression() {
        let compression = Compression::Bzip;
        let entry: Entry = EntryBuilder::new().with_compression(&compression).build();

        assert_eq!(entry.compression, compression);
    }
}
