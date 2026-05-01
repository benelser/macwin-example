//! Cross-platform IPC for macwin-debug-mvp.
//!
//! On **Windows** this becomes a named pipe at `\\.\pipe\<id>`.
//! On **macOS / Linux** it's a Unix domain socket via the system namespace.
//!
//! The same `serve_one` / `send_recv` API runs on every macwin target.

use std::io::{BufRead, BufReader, Write};

use interprocess::local_socket::{
    traits::{Listener, Stream as StreamTrait},
    GenericNamespaced, ListenerOptions, Name, Stream, ToNsName,
};

pub fn pipe_name(id: &str) -> Result<Name<'_>, std::io::Error> {
    id.to_ns_name::<GenericNamespaced>()
}

/// Listen for one client, echo `ack: <msg>`, exit. Used by tests.
pub fn serve_one(id: &str) -> Result<(), std::io::Error> {
    let name = pipe_name(id)?;
    let listener = ListenerOptions::new().name(name).create_sync()?;
    let conn = listener
        .accept()
        .map_err(|e| std::io::Error::other(format!("accept: {e}")))?;
    let mut reader = BufReader::new(conn);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let msg = line.trim_end_matches(&['\r', '\n'][..]).to_string();
    let mut conn = reader.into_inner();
    writeln!(conn, "ack: {msg}")?;
    conn.flush()?;
    Ok(())
}

/// Connect, send a message, read one line back, return it.
pub fn send_recv(id: &str, message: &str) -> Result<String, std::io::Error> {
    let name = pipe_name(id)?;
    let mut conn = Stream::connect(name)?;
    writeln!(conn, "{message}")?;
    conn.flush()?;
    let mut reader = BufReader::new(&mut conn);
    let mut reply = String::new();
    reader.read_line(&mut reply)?;
    Ok(reply.trim_end_matches(&['\r', '\n'][..]).to_string())
}

pub fn info() -> String {
    format!(
        "macwin-debug-mvp {} • {}-{} • {} • ipc={}",
        env!("CARGO_PKG_VERSION"),
        std::env::consts::OS,
        std::env::consts::ARCH,
        if cfg!(debug_assertions) { "debug" } else { "release" },
        if cfg!(windows) { "named-pipes" } else { "unix-sockets" },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn info_well_formed() {
        let s = info();
        assert!(s.starts_with("macwin-debug-mvp "));
        assert!(s.contains("ipc="));
    }
}
