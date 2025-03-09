use std::{
    io::{Read, Write},
    process::Command,
};

fn main() {
    test_stdin();
}

fn test_stdin() {
    let mut head_cmd = Command::new("head");
    head_cmd.arg("-n 2");
    head_cmd.stdin(std::process::Stdio::piped());
    // head_cmd.stdout(std::process::Stdio::piped());

    let mut proc_handle = head_cmd.spawn().unwrap();
    let mut stdin_handle = proc_handle.stdin.take().unwrap();
    
    let input_data = "testline1\nline2".as_bytes();
    _ = stdin_handle.write_all(input_data);

    // _ = proc_handle.wait();

    // let mut out_buffer = String::new();
    // let _ = proc_handle.stdout.unwrap().read_to_string(&mut out_buffer);

    // println!("{}", out_buffer);
}

#[allow(dead_code)]
fn test_command() {
    let mut p1 = Command::new("whoami");

    let result = p1.output();
    if result.is_ok() {
        println!(
            "whoami\n{}",
            String::from_utf8_lossy(&result.unwrap().stdout)
        );
    } else {
        println!("Command failed to execute");
    }

    let mut p1 = Command::new("ls");

    p1.arg("-la");

    let result = p1.output();
    if result.is_ok() {
        println!("ls\n{}", String::from_utf8_lossy(&result.unwrap().stdout));
    } else {
        println!("Command failed to execute");
    }

    // execute a command that does not exist
    let mut p1 = Command::new("xxxxx");

    let result = p1.output();
    if result.is_ok() {
        println!("{}", String::from_utf8_lossy(&result.unwrap().stdout));
    } else {
        println!(
            "Command failed to execute: {}",
            result.unwrap_err().to_string()
        );
    }
}
