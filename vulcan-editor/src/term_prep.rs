use clearscreen::*;

pub(crate) fn term_prep(data:String){
    // Takes file data and converts to a string 
    clearscreen::clear().expect("failed to clear screen"); 
    
    println!("Vulcan Editor: ctrl + x to exit, use arrow keys to navigate."); 
    println!("{}", data);
}
