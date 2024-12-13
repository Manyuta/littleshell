use std::env;
use std::error::Error;
use std::io::{self, Write};
use std::process::Command;

const BUILTIN_COMMANDS: [&str; 3] = ["cd", "help", "exit"];
const BUILTIN_FNS: [fn(&[&str]) -> i32; 3] = [sh_cd, sh_help, sh_exit];

pub fn loop_run() -> Result<(), Box<dyn Error>> {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input)?;

        let trimmed = input.trim();

        let args: Vec<&str> = trimmed.split_whitespace().collect();

        if execute(&args) == 0 {
            break;
        }
    }
    Ok(())
}

fn sh_cd(args: &[&str]) -> i32 {
    if args.len() > 2 {
        eprintln!("lsh: expected argument to \"cd\"");
    } else {
        if let Err(err) = env::set_current_dir(&args[1]) {
            eprintln!("lsh: {}", err);
        }
    }
    1
}

fn sh_help(_args: &[&str]) -> i32 {
    println!();
    println!("Welcome to LSH (Little Shell)!");
    println!("This is a simple shell implementation in Rust.");
    println!();
    println!("Usage:");
    println!("  Type program names and arguments, then press Enter to execute.");
    println!();
    println!("Builtin Commands:");
    println!("  cd [DIR]       Change the current directory to DIR.");
    println!("  help           Show this help message.");
    println!("  exit           Exit the shell.");
    println!();
    println!("External Commands:");
    println!("  Any command available in your system's PATH, such as:");
    println!("    ls, cat, echo, grep, etc.");
    println!();
    println!("For detailed information on external commands, use the 'man' command.");
    println!();

    1
}

fn sh_exit(_args: &[&str]) -> i32 {
    0
}

fn execute(args: &[&str]) -> i32 {
    if args.is_empty() {
        return 1;
    }

    let program = args[0];
    let args = &args[1..];

    for (index, command) in BUILTIN_COMMANDS.iter().enumerate() {
        if program == *command {
            return BUILTIN_FNS[index](args);
        }
    }

    return launch(program, args);
}

fn launch(command: &str, args: &[&str]) -> i32 {
    match Command::new(command).args(args).spawn() {
        Ok(mut child) => {
            // Wait for the child process to finish
            if let Err(err) = child.wait() {
                eprintln!("Error waiting for the child process: {err}");
                return 0;
            }
            return 1;
        }
        Err(err) => {
            eprintln!("Error spawing process: {err}");
            return 0;
        }
    }
}
