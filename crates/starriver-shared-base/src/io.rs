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
    /// 读取的数据超出允许上限（如上传请求体大小限制）
    TooLarge,
    Other(String),
}

impl Display for AsyncReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsyncReaderError::IO(e) => write!(f, "IO error: {e}"),
            AsyncReaderError::TooLarge => write!(f, "payload too large"),
            AsyncReaderError::Other(e) => write!(f, "Other error: {e}"),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait AsyncWriter {
    /// Writes data from the given buffer, returning the number of bytes written.
    ///
    /// 注意：`Ok(n)` 表示**可能只写入了一部分**（`0 < n <= buf.len()`），
    /// 调用方必须处理短写，或直接使用 [`AsyncWriter::write_all`]。
    /// # Arguments
    /// * `buf` - The buffer to write from.
    /// # Returns
    /// * `Ok(n)` - `n` bytes were written.
    /// * `Err` - An error occurred.
    fn write(&mut self, buf: &[u8]) -> impl Future<Output = Result<usize, AsyncWriterError>>;

    /// 将整个 buffer 写完（默认基于 `write` 循环实现，处理短写）。
    fn write_all(&mut self, mut buf: &[u8]) -> impl Future<Output = Result<(), AsyncWriterError>> {
        async move {
            while !buf.is_empty() {
                let n = self.write(buf).await?;
                if n == 0 || n > buf.len() {
                    return Err(AsyncWriterError::Other(
                        "writer made no progress or returned an invalid length".to_string(),
                    ));
                }
                buf = &buf[n..];
            }
            Ok(())
        }
    }

    /// 确保已写入的数据真正落盘（默认 no-op）。
    /// 写入结束后调用方应调用一次，以暴露缓冲实现延迟报告的错误。
    fn flush(&mut self) -> impl Future<Output = Result<(), AsyncWriterError>> {
        std::future::ready(Ok(()))
    }
}

pub enum AsyncWriterError {
    IO(io::Error),
    Other(String),
}

impl Display for AsyncWriterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsyncWriterError::IO(e) => write!(f, "IO error: {e}"),
            AsyncWriterError::Other(e) => write!(f, "Other error: {e}"),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

/// 带逐块钩子的复制。
///
/// 从 `reader` 读出数据，经 `on_chunk` 审核/观察后通过 `write_all` 写入 `writer`，
/// 最后调用 `flush`。成功时返回写入的总字节数。
///
/// # `on_chunk` 的入参
///
/// 签名：`FnMut(&[u8], u64) -> Result<(), E>`
///
/// - `chunk`：本次从 `reader` 读到的**非空**数据块，借用自内部缓冲区 `buf`，
///   只在本次调用期间有效；需要保留请自行拷贝，不要跨调用保存切片。
///   其长度就是本次 `read` 的返回值（返回 `0` 表示 EOF，不会触发回调）。
/// - `written`：本块**之前**已成功写入的累计字节数（`u64`），
///   也就是本块在输出流中的起始偏移；本块写完后才会增加 `chunk.len()`。
///   用它做「已写入 + 本块长度」的限流/计量判断（例如大小上限）。
///
/// # 调用时机与约定
///
/// - 每个非空块调用一次，且在本块写入**之前**；
/// - 返回 `Ok(())`：继续写入本块；
/// - 返回 `Err(e)`：本块**不会被写入**，复制立即中止，
///   错误原样作为 [`CopyStreamError::Hook`] 返回；
/// - 底层 `write_all` 可能分多次写入，但 hook 每块只调用一次；
/// - hook 是同步函数，避免阻塞操作（如网络 I/O）；
/// - 若 hook 成功但随后的 `write_all` 失败，hook 已经观察过该块，
///   但该块可能并未落盘——hook 的副作用应视为「待确认」。
///
/// # Errors
///
/// - [`CopyStreamError::Reader`]：读取失败
/// - [`CopyStreamError::Hook`]：hook 返回错误（本块未写入）
/// - [`CopyStreamError::Writer`]：写入或 flush 失败
pub async fn copy_stream_with_hook<F, E>(
    reader: &mut impl AsyncReader,
    writer: &mut impl AsyncWriter,
    buf: &mut [u8],
    mut on_chunk: F,
) -> Result<u64, CopyStreamError<E>>
where
    F: FnMut(&[u8], u64) -> Result<(), E>,
{
    let mut total = 0u64;
    loop {
        let n = reader.read(buf).await.map_err(CopyStreamError::Reader)?;
        if n == 0 {
            break;
        }
        on_chunk(&buf[..n], total).map_err(CopyStreamError::Hook)?;
        writer
            .write_all(&buf[..n])
            .await
            .map_err(CopyStreamError::Writer)?;
        total += n as u64;
    }
    writer.flush().await.map_err(CopyStreamError::Writer)?;
    Ok(total)
}

/// Copies data from a reader to a writer using a buffer.
pub async fn copy_stream(
    reader: &mut impl AsyncReader,
    writer: &mut impl AsyncWriter,
    buf: &mut [u8],
) -> Result<u64, CopyStreamError> {
    copy_stream_with_hook(reader, writer, buf, |_, _| Ok(())).await
}

/// 复制过程中的错误。
///
/// `E` 是 `on_chunk` 钩子的错误类型，默认 `Infallible`
///（即 [`copy_stream`] 的无钩子版本）。
pub enum CopyStreamError<E = std::convert::Infallible> {
    /// 读取失败
    Reader(AsyncReaderError),
    /// 写入或 `flush` 失败
    Writer(AsyncWriterError),
    /// 钩子返回的错误；此时当前块没有被写入
    Hook(E),
}
