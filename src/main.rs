use nix::unistd::getuid;
use std::env;
mod imut_api;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: {} <command>", args[0]);
        return;
    }

    // TODO: Don't let this be activated if args are 0
    if args[1] == "enable" {
        if getuid().to_string() != "0" {
            println!("You must be root to modify the filesystem");
            return;
        }

        if !imut_api::getmode() {
            imut_api::enterro();
            println!("Filesystem is now immutable");
        } else {
            println!("System is already immutable");
        }
    } else if args[1] == "disable" {
        if getuid().to_string() != "0" {
            println!("You must be root to modify the filesystem");
            return;
        }

        if imut_api::getmode() {
            imut_api::enterrw();
            println!("Filesystem is now mutable");
        } else {
            println!("System is already mutable");
        }

    } else if args[1] == "check" {
        if imut_api::getmode() {
            println!("System is in immutable mode");
        } else {
            println!("System is not in immutable mode");
        }
    } else {
        println!("Invalid argument");
    }

}
