use std::env;
mod api;

fn main() {
    if api::getmode() {
        println!("System is in immutable mode");
    } else {
        println!("System is not in immutable mode");
    }

    let args: Vec<String> = env::args().collect();

    if args[1] == "enable" {
        println!("Enabling immutable mode");
        api::enterro();
    } else if args == ["immutable", "disable"] {
        println!("Disabling immutable mode");
        api::enterrw();
    }
}
