use std::*;
use unindent::unindent;

mod cwd;
mod controls;
mod clean;
//mod input_clean;

fn main() {
    // Variable initizations and declarations 
    let arg: Vec<String> = env::args().collect();
    let mut user_input = String::new();
    

    // Cleans the terminal 
    clean::cleaner();


    // Finds current directory to set filepath for editing 
    let curd = cwd::get_current_working_dir();
    
    let mut filepath:String = curd.to_owned();
    
    let buf:&str = "/";
    let file:&String = &arg[1];
    
    filepath.push_str(&buf);
    filepath.push_str(&file);
    

    // Takes file data and converts to a string
    let data = fs::read_to_string(filepath).expect("Unable to read file");
    print!("{}", data);


    // Infinite loop for editing
    loop {
        //clean::cleaner();
        
        //print!("{}", data);

        io::stdin().read_line(&mut user_input).expect("Failed to read line"); 
        controls::controller(&mut unindent(&mut user_input));
        user_input.clear();
    }
}
