use std::sync::Arc;

use crate::errors::ServerResult;

pub fn parse_stream(buf: &[u8]) -> Vec<Arc<[u8]>> {
    let delimiter = b"\r\n";
    let mut parts = Vec::new();
    let mut start = 0;

    while let Some(pos) = buf[start..]
        .windows(delimiter.len())
        .position(|w| w == delimiter)
    {
        let end = start + pos;
        if end > start {
            parts.push(&buf[start..end]);
        }
        start = end + delimiter.len();
    }

    // Handle remaining bytes after last delimiter
    if start < buf.len() {
        parts.push(&buf[start..]);
    }

    parts
        .into_iter()
        .filter(|x| !x.is_empty())
        .map(|slice| Arc::from(slice))
        .collect()
}