//! Common interfaces for the level backend.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod error;

use std::io;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub use backend_macros::*;
pub use error::*;

/// The length of the size portion of each chunk of data.
pub const LEN_SIZE: usize = 5;

/// The maximum size of data to read at once.
pub const READER_CAPACITY: usize = 1 << 10; // 1 KiB

/// Encodes the size portion of a section of data.
pub fn encode_section_size(mut size: usize) -> [u8; LEN_SIZE] {
    let mut encoded_size = [0u8; LEN_SIZE];

    for i in 0..LEN_SIZE {
        encoded_size[LEN_SIZE - i - 1] = u8::try_from(size % 256).unwrap();
        size >>= 8;
    }

    encoded_size
}

/// Decodes the size portion of a section of data.
pub fn decode_section_size(encoded_size: &[u8; LEN_SIZE]) -> usize {
    let mut size: usize = 0;

    encoded_size.iter().for_each(|val| {
        size <<= 8;
        size += usize::from(*val);
    });

    size
}

/// Reads a section of data from a file.
pub async fn read_section(file: &mut File) -> io::Result<Option<Vec<u8>>> {
    let mut size_buffer = [0u8; LEN_SIZE];

    let n = file.read(&mut size_buffer).await?;

    if n == 0 {
        return Ok(None);
    }

    let decoded_size = decode_section_size(&size_buffer);
    let mut buffer = vec![0u8; decoded_size];

    let n = file.read(&mut buffer).await?;

    if n != decoded_size {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "read fewer bytes from file than expected",
        ));
    }

    Ok(Some(buffer))
}

/// Writes a section of data to a file.
pub async fn write_section(file: &mut File, data: &[u8]) -> io::Result<()> {
    let encoded_size = encode_section_size(data.len());

    file.write_all(&encoded_size).await?;
    file.write_all(data).await?;

    Ok(())
}

/// Copy the contents of one file to another in chunks.
pub async fn copy_file_in_chunks(src: &mut File, dest: &mut File) -> io::Result<()> {
    let mut buffer = [0u8; READER_CAPACITY];

    loop {
        let n = src.read(&mut buffer).await?;

        if n == 0 {
            break;
        }

        dest.write_all(&buffer[..n]).await?;
    }

    dest.flush().await?;

    Ok(())
}
