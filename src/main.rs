use std::env;
use std::process;
use std::thread;
use std::time::Duration;

fn main() {

    // Accumulate parsed time components as seconds
    let mut remind_seconds = 0;

    for arg in env::args().skip(1){
        match arg[0..arg.len()-1].parse::<u64>() {
            Ok(duration) => {
                match arg[arg.len()-1..arg.len()].as_ref() {
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
    println!("Reminder set for {} seconds", remind_seconds);

    let duration = Duration::new(remind_seconds, 0);
    thread::sleep(duration);
    // TODO how to daemonize a process? I don't want to leave the console hanging here...

    // TODO can I get a real beep? Found beep lib, maybe that.
    println!("Ding!");
}

// Show usage string, optionally exit if exit_code is non-zero
fn usage(exit_code: i32) {
    println!("remindme [duration][unit]");
    println!("\n");
    println!("\tduration\t\tInteger duration");
    println!("\tunit\t\tTime unit character from [s]econds, [m]inutes, or [h]ours");
    println!("Example: Set a reminder for 1 hour, 30 minutes, and 20 seconds");
    println!("\tremindme 1h 30m 20s");
    if exit_code > 0{
        process::exit(exit_code)
    }
}
