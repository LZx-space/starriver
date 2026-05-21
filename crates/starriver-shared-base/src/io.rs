use std::{fmt::Display, io};

pub trait AsyncReader {
    /// Reads data into the given buffer, returning the number of bytes read.
    /// # Arguments
    /// * `buf` - The buffer to read into.
    /// # Returns
    /// * `Ok(n)` - `n` bytes were read, 0 is EOF
    /// * `Err` - An error occurred.
    fn read(&mut self, buf: &mut [u8]) -> impl Future<Output = Result<usize, AsyncReaderError>>;
}

pub enum AsyncReaderError {
    IO(io::Error),
    Other(String),
}

impl Display for AsyncReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsyncReaderError::IO(e) => write!(f, "IO error: {}", e),
            AsyncReaderError::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait AsyncWriter {
    /// Writes data from the given buffer, returning the number of bytes written.
    /// # Arguments
    /// * `buf` - The buffer to write from.
    /// # Returns
    /// * `Ok(n)` - `n` bytes were written.
    /// * `Err` - An error occurred.
    fn write(&mut self, buf: &[u8]) -> impl Future<Output = Result<usize, AsyncWriterError>>;
}

pub enum AsyncWriterError {
    IO(io::Error),
    Other(String),
}

impl Display for AsyncWriterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsyncWriterError::IO(e) => write!(f, "IO error: {}", e),
            AsyncWriterError::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Copies data from a reader to a writer using a buffer.
pub async fn copy_stream(
    reader: &mut impl AsyncReader,
    writer: &mut impl AsyncWriter,
    buf: &mut [u8],
) -> Result<u64, CopyStreamError> {
    let mut total = 0u64;
    loop {
        let n = reader.read(buf).await.map_err(CopyStreamError::Reader)?;
        if n == 0 {
            break;
        }
        writer
            .write(&buf[..n])
            .await
            .map_err(CopyStreamError::Writer)?;
        total += n as u64;
    }
    Ok(total)
}

pub enum CopyStreamError {
    Reader(AsyncReaderError),
    Writer(AsyncWriterError),
}
