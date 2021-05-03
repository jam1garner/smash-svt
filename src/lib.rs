//! A Rust library for working with `sound_volume_fighter_num_table.fnv` files from Smash Ultimate.

use binread::{BinRead, BinReaderExt, derive_binread};
use binwrite::{BinWrite, WriterOption};

use std::fs::File;
use std::path::Path;
use std::io::{self, Read, Seek, Write, BufReader, BufWriter};

#[cfg(feature = "derive_serde")]
use serde::{Serialize, Deserialize};

pub use binread::{BinResult as Result, Error};

#[derive_binread]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
#[br(magic = b"SVT\0\x01\0\0\0")]
pub struct SvtFile (
    #[br(temp)]
    u32,

    #[br(count = self_0)]
    Vec<Entry>,
);

impl BinWrite for SvtFile {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> io::Result<()> {
        let entries = self.0.clone();

        (
            b"SVT\0\x01\0\0\0",
            self.0.len() as u32,
            entries
        ).write_options(writer, options)
    }
}

/// An entry in the sound volume table
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite, Debug, Clone)]
pub struct Entry {
    id: u32,
    knobs: [f32; 4],
}

impl SvtFile {
    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<Self> {
        reader.read_le()
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        BufReader::new(File::open(path)?).read_le()
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.write(&mut BufWriter::new(File::create(path)?))
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.write_options(writer, &binwrite::writer_option_new!(endian: binwrite::Endian::Little))
            .map_err(Into::into)
    }

    pub fn new(entries: Vec<Entry>) -> Self {
        SvtFile(entries)
    }

    pub fn entries(&self) -> &[Entry] {
        &self.0
    }

    pub fn entries_mut(&mut self) -> &mut Vec<Entry> {
        &mut self.0
    }
}
