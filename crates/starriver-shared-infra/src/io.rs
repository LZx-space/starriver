use std::{
    io,
    path::{Path, PathBuf},
};

use axum::{body::Bytes, extract::multipart::Field, http::StatusCode};
use starriver_shared_base::io::{
    AsyncReader, AsyncReaderError, AsyncWriter, AsyncWriterError, FileAsyncWriter,
    FileWriterFactory, NamedAsyncReader,
};
use tokio::{fs::File, io::AsyncWriteExt};
use tracing::info;

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
                Err(e) => {
                    return Err(if e.status() == StatusCode::PAYLOAD_TOO_LARGE {
                        AsyncReaderError::TooLarge
                    } else {
                        AsyncReaderError::Other(e.to_string())
                    });
                }
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

impl<'a> NamedAsyncReader for MultipartFieldAsyncReader<'a> {
    fn file_name(&self) -> &str {
        self.field.file_name().unwrap_or_default()
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////

pub struct TokioFileAsyncWriter {
    file: File,
    path: PathBuf,
}

impl TokioFileAsyncWriter {
    /// 创建文件
    pub async fn new(path: impl AsRef<Path>) -> io::Result<Self> {
        Ok(Self {
            path: path.as_ref().to_path_buf(),
            file: File::create(path).await?,
        })
    }
}

impl AsyncWriter for TokioFileAsyncWriter {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, AsyncWriterError> {
        self.file.write(buf).await.map_err(AsyncWriterError::IO)
    }

    async fn flush(&mut self) -> Result<(), AsyncWriterError> {
        self.file.flush().await.map_err(AsyncWriterError::IO)
    }
}

impl FileAsyncWriter for TokioFileAsyncWriter {
    async fn rename(&mut self, to: impl AsRef<std::path::Path>) -> Result<(), AsyncWriterError> {
        tokio::fs::rename(&self.path, to.as_ref())
            .await
            .map_err(AsyncWriterError::IO)?;
        self.path = to.as_ref().to_path_buf();
        Ok(())
    }

    async fn remove(&self) -> Result<(), AsyncWriterError> {
        tokio::fs::remove_file(&self.path)
            .await
            .map_err(AsyncWriterError::IO)
    }
}

pub struct DefaultFileWriterFactory;

impl FileWriterFactory for DefaultFileWriterFactory {
    type Writer = TokioFileAsyncWriter;

    async fn create(&self, path: &std::path::Path) -> Result<Self::Writer, AsyncWriterError> {
        info!("new async writer");
        let writer = TokioFileAsyncWriter::new(path)
            .await
            .map_err(AsyncWriterError::IO)?;
        Ok(writer)
    }
}
