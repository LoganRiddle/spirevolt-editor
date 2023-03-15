use std::process::Command; 
use std::io;
use std::io::Read;
use std::env;
use std::fs;
mod cwd;
mod controls;

fn main() {
    let arg: Vec<String> = env::args().collect();
    let mut controller:String;

    // Clears the terminal for Linux, Mac, and Powershell on Windows
    let output = Command::new("clear").output().expect("clear command failed to start"); 
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));    
    assert!(output.status.success());


    // Finds current directory to set filepath for editing 
    let curd = cwd::get_current_working_dir();
    let mut filepath:String = curd.to_owned();
    let buf:&str = "/";
    let file:&String = &arg[1];
    filepath.push_str(&buf);
    filepath.push_str(&file);
    //println!("Filepath: {}", filepath);


    // Takes file data and converts to a string
    let data = fs::read_to_string(filepath).expect("Unable to read file");
    println!("{}", data);


    // Infinite loop for editing
    let mut input = io::stdin();
    loop {
        input = io::stdin();
        controls::controls(input.to_string());
    }
}
