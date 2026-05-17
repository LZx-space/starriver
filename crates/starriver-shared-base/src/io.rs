pub trait AsyncReader {
    /// Reads data into the given buffer, returning the number of bytes read.
    /// # Arguments
    /// * `buf` - The buffer to read into.
    /// # Returns
    /// * `Ok(n)` - `n` bytes were read, 0 is EOF
    /// * `Err` - An error occurred.
    fn read(&mut self, buf: &mut [u8]) -> impl Future<Output = Result<usize, AsyncReaderError>>;
}

pub enum AsyncReaderError {}
