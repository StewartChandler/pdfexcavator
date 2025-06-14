use std::{
    fmt::{Debug, Display},
    fs::File,
    io::{self, BufReader, Read, Seek},
    path::Path,
};

use no_panic::no_panic;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PDFInitializationError {
    #[error("unable to open file")]
    FileOpen(#[from] io::Error),
    #[error("could not recognize version string, may not be a pdf file")]
    BadVersionIdentifier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PDFVersion {
    PDF1_0,
    PDF1_1,
    PDF1_2,
    PDF1_3,
    PDF1_4,
    PDF1_5,
    PDF1_6,
    PDF1_7,
    PDF2_0,
}

impl Display for PDFVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PDFVersion::PDF1_0 => f.write_str("1.0"),
            PDFVersion::PDF1_1 => f.write_str("1.1"),
            PDFVersion::PDF1_2 => f.write_str("1.2"),
            PDFVersion::PDF1_3 => f.write_str("1.3"),
            PDFVersion::PDF1_4 => f.write_str("1.4"),
            PDFVersion::PDF1_5 => f.write_str("1.5"),
            PDFVersion::PDF1_6 => f.write_str("1.6"),
            PDFVersion::PDF1_7 => f.write_str("1.7"),
            PDFVersion::PDF2_0 => f.write_str("2.0"),
        }
    }
}

/// A generic reader struct for PDFs
#[derive(Debug)]
pub struct PDFReader<T>
where
    T: Debug + Read + Seek,
{
    #[allow(unused)]
    inner: BufReader<T>,
    #[allow(unused)]
    version: PDFVersion,
}

impl<T> PDFReader<T>
where
    T: Debug + Read + Seek,
{
    #[no_panic]
    fn from_bufreader(mut bf: BufReader<T>) -> Result<Self, PDFInitializationError> {
        let mut buf = [0u8; 10];
        bf.read(&mut buf)?;

        let version = match &buf {
            b"%PDF-1.0\r\n" | &[b'%', b'P', b'D', b'F', b'-', b'1', b'.', b'0', b'\n', _] => {
                Ok(PDFVersion::PDF1_0)
            }
            b"%PDF-1.1\r\n" | &[b'%', b'P', b'D', b'F', b'-', b'1', b'.', b'1', b'\n', _] => {
                Ok(PDFVersion::PDF1_1)
            }
            b"%PDF-1.2\r\n" | &[b'%', b'P', b'D', b'F', b'-', b'1', b'.', b'2', b'\n', _] => {
                Ok(PDFVersion::PDF1_2)
            }
            b"%PDF-1.3\r\n" | &[b'%', b'P', b'D', b'F', b'-', b'1', b'.', b'3', b'\n', _] => {
                Ok(PDFVersion::PDF1_3)
            }
            b"%PDF-1.4\r\n" | &[b'%', b'P', b'D', b'F', b'-', b'1', b'.', b'4', b'\n', _] => {
                Ok(PDFVersion::PDF1_4)
            }
            b"%PDF-1.5\r\n" | &[b'%', b'P', b'D', b'F', b'-', b'1', b'.', b'5', b'\n', _] => {
                Ok(PDFVersion::PDF1_5)
            }
            b"%PDF-1.6\r\n" | &[b'%', b'P', b'D', b'F', b'-', b'1', b'.', b'6', b'\n', _] => {
                Ok(PDFVersion::PDF1_6)
            }
            b"%PDF-1.7\r\n" | &[b'%', b'P', b'D', b'F', b'-', b'1', b'.', b'7', b'\n', _] => {
                Ok(PDFVersion::PDF1_7)
            }
            b"%PDF-2.0\r\n" | &[b'%', b'P', b'D', b'F', b'-', b'2', b'.', b'0', b'\n', _] => {
                Ok(PDFVersion::PDF2_0)
            }
            _ => Err(PDFInitializationError::BadVersionIdentifier),
        }?;

        Ok(Self { inner: bf, version })
    }
}

impl PDFReader<File> {
    /// Creates a new `PDFReader` object from a file located at the the path specified by `path`
    pub fn from_file_path<P: AsRef<Path>>(path: P) -> Result<Self, PDFInitializationError> {
        Self::from_bufreader(BufReader::new(File::open(path)?))
    }
}
