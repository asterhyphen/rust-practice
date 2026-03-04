use std::io::{self, Write};
use std::process::Command;

fn run(cmd: &str, args: &[&str]) {
    let output = Command::new(cmd)
        .args(args)
        .output();

    match output {
        Ok(o) => {
            print!("{}", String::from_utf8_lossy(&o.stdout));
            eprint!("{}", String::from_utf8_lossy(&o.stderr));
        }
        Err(e) => println!("error: {}", e),
    }
}

fn main() {
    loop {
        print!("ctf> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() { continue; }

        match parts[0] {

            "john" => {
                run("john", &parts[1..]);
            }

            "hashcat" => {
                run("hashcat", &parts[1..]);
            }

            "nmap" => {
                run("nmap", &parts[1..]);
            }

            "hydra" => {
                run("hydra", &parts[1..]);
            }

            "binwalk" => {
                run("binwalk", &parts[1..]);
            }

            "exit" => break,

            _ => println!("unknown command")
        }
    }
}