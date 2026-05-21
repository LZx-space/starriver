use std::io;

use axum::{body::Bytes, extract::multipart::Field};
use starriver_shared_base::io::{AsyncReader, AsyncReaderError, AsyncWriter, AsyncWriterError};
use tokio::{fs::File, io::AsyncWriteExt};

/// axum multipart field async reader
pub struct MultipartFieldAsyncReader<'a> {
    field: &'a mut Field<'a>,
    last_chunk_remain: Option<Bytes>,
}

impl<'a> MultipartFieldAsyncReader<'a> {
    pub fn new(field: &'a mut Field<'a>) -> Self {
        Self {
            field,
            last_chunk_remain: None,
        }
    }
}

impl<'a> AsyncReader for MultipartFieldAsyncReader<'a> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, AsyncReaderError> {
        // 1. 取数据：优先余留，否则拉下一个 chunk
        let chunk = match self.last_chunk_remain.take() {
            Some(remain) => remain,
            None => match self.field.chunk().await {
                Ok(Some(chunk)) => chunk,
                Ok(None) => return Ok(0),
                Err(e) => return Err(AsyncReaderError::Other(e.to_string())),
            },
        };

        // 2. 复制到 caller 的 buf
        let n = chunk.len().min(buf.len());
        buf[..n].copy_from_slice(&chunk[..n]);

        // 3. 未消费完的存回
        if n < chunk.len() {
            self.last_chunk_remain = Some(chunk.slice(n..));
        }

        Ok(n)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////

pub struct TokioFileAsyncWriter {
    file: File,
}

impl TokioFileAsyncWriter {
    pub async fn new(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
        Ok(Self {
            file: File::create(path).await?,
        })
    }
}

impl AsyncWriter for TokioFileAsyncWriter {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, AsyncWriterError> {
        let n = self.file.write(buf).await.map_err(AsyncWriterError::IO)?;
        if n == 0 {
            self.file.flush().await.map_err(AsyncWriterError::IO)?;
        }
        Ok(n)
    }
}
