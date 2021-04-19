//! A Rust library for working with `commonsoundtable.csb` files from Smash Ultimate. This allows
//! for modifying which common sounds are loaded for which characters

use binread::{BinRead, BinReaderExt, derive_binread};
use binwrite::{BinWrite, WriterOption};

use std::fs::File;
use std::path::Path;
use std::io::{self, Read, Seek, Write, BufReader, BufWriter};

#[cfg(feature = "derive_serde")]
use serde::{Serialize, Deserialize};

pub use hash40;
use hash40::Hash40;

pub use binread::{BinResult as Result, Error};

#[derive_binread]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
#[br(magic = b"CSB\0\x01\0\0\0")]
pub struct CsbFile (
    #[br(temp)]
    u16,

    #[br(temp)]
    u16,

    #[br(count = self_1, args(self_0))]
    Vec<Entry>,
);

impl BinWrite for CsbFile {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> io::Result<()> {
        let entries = self.0.clone();
        //entries.sort_unstable_by(|a, b| a.character_names.cmp(&b.tone_name));

        (
            b"CSB\0\x01\0\0\0",
            self.0.get(0).map(|entry| entry.sounds.len()).unwrap_or(0) as u16,
            self.0.len() as u16,
            entries
        ).write_options(writer, options)
    }
}

/// An entry representing a character's
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite, Debug, Clone)]
#[br(import(sound_count: u16))]
pub struct Entry {
    #[br(map = Hash40)]
    #[binwrite(preprocessor(hash40_to_u64))]
    pub character_name: Hash40,

    #[br(count = sound_count, map = hash40_vec)]
    #[binwrite(preprocessor(vec_hash40))]
    pub sounds: Vec<Hash40>,
}

fn hash40_to_u64(&Hash40(x): &Hash40) -> u64 {
    x
}

fn hash40_vec(x: Vec<u64>) -> Vec<Hash40> {
    x.into_iter().map(Hash40).collect()
}

fn vec_hash40(x: &Vec<Hash40>) -> Vec<u64> {
    x.iter().map(|&Hash40(x)| x).collect()
}

impl CsbFile {
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
        CsbFile(entries)
    }

    pub fn entries(&self) -> &[Entry] {
        &self.0
    }

    pub fn entries_mut(&mut self) -> &mut Vec<Entry> {
        &mut self.0
    }
}
