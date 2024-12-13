use littleshell::loop_run;

fn main() {
    if let Err(e) = loop_run() {
        eprintln!("Application error: {e}");
    }
}
