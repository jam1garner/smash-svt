//! # svt_lib
//!
//! svt_lib is a library for reading and writing `sound_volume_table.svt` files from Super Smash Bros. for Nintendo 3DS and Wii U and Super Smash Bros. Ultimate.
use std::{
    fs,
    io::{Cursor, Read, Seek, Write},
    path::Path,
};

use binrw::{binrw, BinReaderExt, BinResult, BinWrite};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The container type for groups of audio gain parameters.
#[binrw]
#[brw(magic = b"SVT\0", little)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug)]
pub struct SvtFile {
    #[br(temp)]
    #[bw(calc = 1u32)]
    unk1: u32,

    #[br(temp)]
    #[bw(calc = entries.len() as u32)]
    entry_count: u32,

    #[br(count = entry_count)]
    pub entries: Vec<SvtEntry>,
}

impl SvtFile {
    /// Reads the data from the given reader.
    pub fn read<R: Read + Seek>(reader: &mut R) -> BinResult<Self> {
        let svt = reader.read_le::<Self>()?;

        Ok(svt)
    }

    /// Reads the data from the given file path.
    pub fn from_file<P: AsRef<Path>>(path: P) -> BinResult<Self> {
        let mut file = Cursor::new(fs::read(path)?);
        let svt = file.read_le::<Self>()?;

        Ok(svt)
    }

    /// Writes the data to the given writer.
    pub fn write<W: Write + Seek>(&self, writer: &mut W) -> BinResult<()> {
        self.write_le(writer)
    }

    /// Writes the data to the given file path.
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> BinResult<()> {
        let mut cursor = Cursor::new(Vec::new());

        self.write_le(&mut cursor)?;
        fs::write(path, cursor.get_mut())?;

        Ok(())
    }
}

/// A group of audio gain parameters.
#[binrw]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug)]
pub struct SvtEntry {
    pub id: u32,
    pub knobs: [f32; 4],
}
