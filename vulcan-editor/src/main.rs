use std::process::Command; 
use std::io;
use std::io::prelude::*;

fn main() {
    // Clears the terminal for Linux, Mac, and Powershell on Windows
    let output = Command::new("clear")
                         .output()
                         .expect("clear command failed to start"); 

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    
    assert!(output.status.success());
    //

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}
