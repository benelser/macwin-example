//! Cross-platform IPC integration tests.

use std::io;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

use macwin_example::{info, send_recv, serve_one};

fn unique_id(label: &str) -> String {
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let n = SEQ.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    format!("macwin-example-{label}-{pid}-{n}")
}

fn spawn_server(id: &str) -> thread::JoinHandle<io::Result<()>> {
    let id = id.to_owned();
    let h = thread::spawn(move || serve_one(&id));
    thread::sleep(Duration::from_millis(80));
    h
}

#[test]
fn info_mentions_arch() {
    assert!(info().contains(std::env::consts::ARCH));
}

#[test]
fn ipc_roundtrip_basic() {
    let id = unique_id("basic");
    let server = spawn_server(&id);
    assert_eq!(send_recv(&id, "hello").unwrap(), "ack: hello");
    server.join().unwrap().unwrap();
}

#[test]
fn ipc_roundtrip_unicode() {
    let id = unique_id("unicode");
    let server = spawn_server(&id);
    let msg = "café 🚀 пример";
    assert_eq!(send_recv(&id, msg).unwrap(), format!("ack: {msg}"));
    server.join().unwrap().unwrap();
}
