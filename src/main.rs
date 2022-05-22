extern crate colour;

use std::io::{self, Write};
use std::env::{set_current_dir, current_dir};
use colour::{e_dark_red_ln};

fn exec_cd(args: Vec<&str>) -> u8 {
    if args.len() == 1 {
        eprintln!("cd: missing argument");
        1
    } else {
        match set_current_dir(args[1]) {
            Ok(_) => 1,
            Err(_) =>  {
                e_dark_red_ln!("cd: no such file or directory: {}", args[1]);
                1
            },
        }
    }
}

fn exec_exit(_args: Vec<&str>) -> u8 {
    0
}

fn exec_command(args: Vec<&str>) -> u8 {
    let commands = vec!["cd", "exit"];
    let command_runners: Vec<&dyn Fn(Vec<&str>) -> u8> = vec![&exec_cd, &exec_exit];

    for (i, command) in commands.iter().enumerate() {
        if &args[0] == command {
            return (command_runners[i])(args);
        }
    }

    e_dark_red_ln!("{}: command not found", args[0]);
    1
}

fn shell_loop() {
    loop {
        print!("yamsh ({})\n> ", current_dir().unwrap().to_str().unwrap());
        _ = io::stdout().flush();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let args = line.split_whitespace().collect::<Vec<&str>>();
        
        let status = exec_command(args);

        if status == 0 {
            break;
        }
    }
}

fn main() {
    shell_loop();
}
