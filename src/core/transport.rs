/// Binary frame transport helpers.
///
/// The Go core communicates over a local socket / named pipe using a simple
/// length-prefixed binary frame protocol: `[4 bytes LE length][payload]`.
use std::io::{self, Read, Write};

/// Write a length-prefixed frame.
pub fn write_frame(mut writer: impl Write, data: &[u8]) -> io::Result<()> {
    let len = data.len() as u32;
    writer.write_all(&len.to_le_bytes())?;
    writer.write_all(data)?;
    writer.flush()
}

/// Read a single length-prefixed frame.
pub fn read_frame(mut reader: impl Read) -> io::Result<Vec<u8>> {
    let mut len_buf = [0u8; 4];
    reader.read_exact(&mut len_buf)?;
    let len = u32::from_le_bytes(len_buf) as usize;

    // Guard against unreasonably large frames
    if len > 16 * 1024 * 1024 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("frame too large: {len} bytes"),
        ));
    }

    let mut payload = vec![0u8; len];
    reader.read_exact(&mut payload)?;
    Ok(payload)
}
