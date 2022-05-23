extern crate colour;
extern crate fork;
extern crate nix;

use std::io::{self, Write};
use std::env::{set_current_dir, current_dir};
use std::process::Command;
use std::os::unix::process::CommandExt;
use colour::{e_dark_red_ln};
use fork::{fork, Fork};
use nix::sys::wait::{waitpid, WaitStatus};

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

fn fork_and_execute(args: Vec<&str>) -> u8 {
    match fork() {
        Ok(Fork::Parent(child)) => {
            loop {
                match waitpid(child, None) {
                    Ok(status) => {
                        match status {
                            WaitStatus::Exited(_, _) | WaitStatus::Signaled(_, _, _) => {
                                break;
                            },
                            _ => {
                                continue;
                            }
                        }
                    },
                    Err(_) => {
                        e_dark_red_ln!("Fork error");
                    }
                }
            }
        },
        Ok(Fork::Child) => {
            let mut runnable_args = args.clone();

            if args.len() == 1 {
                let program_path = args[0];

                runnable_args = Vec::new();
                if program_path.ends_with(".yas") {
                    runnable_args.push("yamasm");
                } else if program_path.ends_with(".out") {
                    runnable_args.push("yamini");
                } else {
                    runnable_args.push("yamini");
                }

                runnable_args.push(program_path);
            }

            let _err = Command::new(runnable_args[0])
                .args(&runnable_args[1..])
                .exec();
        },
        Err(_) => {
            e_dark_red_ln!("Fork failed");
        }
    }

    1
}

fn exec_command(args: Vec<&str>) -> u8 {
    let commands = vec!["cd", "exit"];
    let command_runners: Vec<&dyn Fn(Vec<&str>) -> u8> = vec![&exec_cd, &exec_exit];

    for (i, command) in commands.iter().enumerate() {
        if &args[0] == command {
            return (command_runners[i])(args);
        }
    }

    fork_and_execute(args)
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
