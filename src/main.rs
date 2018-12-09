use std::env;
use std::process;
use std::process::Command;
use std::thread;
use std::time::Duration;

#[cfg(windows)] extern crate winapi;
use std::io::Error;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    // Need at least 3 args: self [in|daemon] [p1 p2 ... pn]
    if args.len() < 3 {
        usage(1);
    }

    match args[1].as_ref() {
        "daemon" => run_daemon(args),
        "in" => make_reminder(args),
        "help" => usage(0),
        _ => usage(1),
    }
}

// Executes daemon function after waiting seconds_delay
fn run_daemon(args: Vec<String>) {

    println!("Received daemon command {:?}", args);

    // No delay period specified
    if args.len() != 3{
        process::exit(1)
    }

    match args[2].parse::<u64>() {
        Ok(n) => {

            println!("Daemon received delay of {:?} seconds", n);

            let duration = Duration::new(n, 0);
            thread::sleep(duration);

            // TODO can I get a real beep? Found beep lib, maybe that.
            let _result = print_message("Reminder, something something");

            process::exit(0);
        }
        Err(_e) => process::exit(1)
    }
}

fn make_reminder(args: Vec<String>) {
    // Accumulate parsed time components as seconds
    let mut remind_seconds = 0;

    if args.len() < 2 {
        return
    }

    for arg in &args[2..] {
        match arg[..arg.len() - 1].parse::<u64>() {
            Ok(duration) => {
                match arg[arg.len() - 1..arg.len()].as_ref() {
                    "s" => remind_seconds += duration,
                    "m" => remind_seconds += duration * 60,
                    "h" => remind_seconds += duration * 60 * 60,
                    _ => {
                        eprintln!("error: unsupported time unit: {}", arg);
                        usage(1);
                    }
                }
            }
            Err(_e) => {
                eprintln!("error: invalid duration component: {}", arg);
                usage(1);
            }
        }
    }

    if let Some(command) = args.get(0) {
        let child = Command::new(command)
            .arg("daemon")
            .arg(remind_seconds.to_string())
            .spawn()
            .unwrap();

        println!("Reminder set for {} seconds on child process {}", remind_seconds, child.id());
    }
}

// Show usage string, optionally exit if exit_code is non-zero
fn usage(exit_code: i32) {
    println!("\nremindme in [duration][unit] ...");
    println!("[args]");
    println!("\thelp\t\tShow this message");
    println!("\tduration\t\tInteger duration");
    println!("\tunit\t\t\tTime unit character from [s]econds, [m]inutes, or [h]ours");
    println!("[example] :: Set a reminder for 1 hour, 30 minutes, and 20 seconds");
    println!("\tremindme in 1h 30m 20s");
    if exit_code > 0 {
        process::exit(exit_code)
    }
}

#[cfg(windows)]
fn print_message(msg: &str) -> Result<i32, Error> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{MB_OK, MessageBoxW};
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)
    };
    if ret == 0 { Err(Error::last_os_error()) }
        else { Ok(ret) }
}
#[cfg(not(windows))]
fn print_message(msg: &str) -> Result<(), Error> {
    println!("{}", msg);
    Ok(())
}