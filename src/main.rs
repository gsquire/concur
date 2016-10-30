extern crate docopt;
extern crate rustc_serialize;

use std::thread;
use std::process::Command;
use std::sync::Arc;
use std::sync::mpsc;

use docopt::Docopt;

const USAGE: &'static str = "
Usage:
    concur [options] <num-times> <binary> <binary-args>...
    concur (-h | --help)

Options:
    -j, --num-threads ARG  Number of threads to use while running the command.
                           Must be less than than or equal to the repeat number.
";

#[derive(Clone, Debug, RustcDecodable)]
struct Args {
    arg_num_times: i32,
    arg_binary: String,
    arg_binary_args: Vec<String>,
    flag_num_threads: Option<i32>,
}

// Evenly divide the work based on the thread count argument and the number of repeats.
fn divide_work(num_threads: i32, num_repeat: i32) -> (i32, i32) {
    ((num_repeat / num_threads), (num_repeat % num_threads))
}

fn print_out(data: Vec<u8>) {
    print!("{}", String::from_utf8(data).expect("invalid UTF-8"));
}

fn run_command(args: &Args) -> Vec<u8> {
    let c = Command::new(&args.arg_binary)
        .args(&args.arg_binary_args)
        .output()
        .expect("failed to execute command");

    c.stdout
}

// Run the command either synchronously or asynchronously depending on flags provided.
fn repeat(args: &Args) {
    if let Some(num_threads) = args.flag_num_threads {
        if num_threads > args.arg_num_times || num_threads <= 0 {
            println!("{}", USAGE);
            return;
        }

        let shared_args = Arc::new(args.clone());
        let (tx, rx) = mpsc::channel();
        let (w, r) = divide_work(num_threads, args.arg_num_times);

        // Divide the work evenly amongst new threads.
        for _ in 0..num_threads {
            let local_args = shared_args.clone();
            let trans = tx.clone();

            thread::spawn(move || {
                for _ in 0..w {
                    trans.send(run_command(&local_args)).unwrap();
                }
            });
        }

        // Perform the remainder in the main thread.
        for _ in 0..r {
            print_out(run_command(args));
        }

        // Print the output from the threads.
        for _ in 0..num_threads * w {
            print_out(rx.recv().unwrap());
        }

        // The processing is done.
        return;
    }

    // The synchronous path.
    for _ in 0..args.arg_num_times {
        print_out(run_command(args));
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    repeat(&args);
}
