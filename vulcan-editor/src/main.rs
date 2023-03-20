use std::*;
use std::io::{stdout, Write};
use unindent::unindent;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::*;
//use terminal_size::{Width, Height, terminal_size};
use clearscreen::*;

mod cwd;


fn main() {
    // Variable initizations and declarations 
    let arg: Vec<String> = env::args().collect();
    //let h = 0;
    let mut char_index = 0;

    // Finds current directory to set filepath for editing 
    let curd = cwd::get_current_working_dir();
    
    let mut filepath:String = curd.to_owned();
    
    let buf:&str = "/";
    let file:&String = &arg[1];
   
    // Appends to the filepath string 
    filepath.push_str(&buf);
    filepath.push_str(&file);
    

    // Takes file data and converts to a string
    let data = fs::read_to_string(filepath).expect("Unable to read file");
    

    // Finds the terminal dimensions
    //let size = terminal_size();
            
    //going into raw mode
    enable_raw_mode().unwrap();

    //clearing the screen, going to top left corner and printing welcoming message
    clearscreen::clear().expect("failed to clear screen"); 

    //Height(h);

    //execute!(stdout(), cursor::MoveTo(0, h)).unwrap();
    println!("Vulcan Editor: ctrl + x to exit, use arrow keys to navigate."); 
   
    // Sets the cursor below the menu and prints file contents 
    execute!(stdout(), cursor::MoveTo(0, 1)).unwrap();
    writeln!(stdout(), "{}", data).unwrap();

    execute!(stdout(), cursor::MoveTo(0, 1)).unwrap();


    //key detection
    loop {
        //matching the key
        match read().unwrap() {
            // Directional Controls
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                ..
            }) => execute!(stdout(), cursor::MoveUp(1)).unwrap(),

            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => execute!(stdout(), cursor::MoveLeft(1)).unwrap(),

            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => execute!(stdout(), cursor::MoveDown(1)).unwrap(),

            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => execute!(stdout(), cursor::MoveRight(1)).unwrap(),
            
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                ..
            }) => print!(" "),


            // Exit without saving
            Event::Key(KeyEvent {
                code: KeyCode::Char('x'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => break,
            
            _ => (),
        }
    }

    clearscreen::clear().expect("failed to clear screen"); 
 
    //disabling raw mode
    disable_raw_mode().unwrap();
}
