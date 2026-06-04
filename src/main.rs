use std::env;
use std::process::ExitCode;
use std::thread;
use std::time::Duration;

use macwin_example::{info, send_recv, serve_one};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let cmd = args.get(1).map(String::as_str).unwrap_or("info");

    match cmd {
        "info" => {
            println!("{}", info());
            ExitCode::SUCCESS
        }
        "ipc" => {
            let id = format!("macwin-example-{}", std::process::id());
            let id_for_server = id.clone();
            let server = thread::spawn(move || serve_one(&id_for_server));
            thread::sleep(Duration::from_millis(80));
            let payload = args.get(2).map(String::as_str).unwrap_or("hello");
            match send_recv(&id, payload) {
                Ok(reply) => println!("client got: {reply}"),
                Err(e) => {
                    eprintln!("client error: {e}");
                    return ExitCode::FAILURE;
                }
            }
            if let Err(e) = server.join().unwrap_or(Err(std::io::Error::other("join"))) {
                eprintln!("server error: {e}");
                return ExitCode::FAILURE;
            }
            ExitCode::SUCCESS
        }
        other => {
            eprintln!("unknown subcommand: {other}");
            eprintln!("usage: macwin-example <info | ipc [message]>");
            ExitCode::from(2)
        }
    }
}
