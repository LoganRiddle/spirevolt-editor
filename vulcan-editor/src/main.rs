use std::*;
use std::io::{stdout, Write};
use unindent::unindent;
use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::*;

mod cwd;
//mod controls;
mod clean;


fn main() {
    // Variable initizations and declarations 
    let arg: Vec<String> = env::args().collect();
    let mut user_input = String::new();
    //let mut stdout = stdout();
    

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
    
    
    //going into raw mode
    enable_raw_mode().unwrap();

    //clearing the screen, going to top left corner and printing welcoming message
    user_input.clear();
    execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();
    println!("Vulcan Editor: ctrl + x to exit, use arrow keys to navigate."); 
    
    execute!(stdout(), cursor::MoveTo(0, 1)).unwrap();
    println!("{}", data);
    execute!(stdout(), cursor::MoveTo(0, 1)).unwrap();

    //key detection
    loop {
        //println!("{}", data);

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


            // Exit without saving
            Event::Key(KeyEvent {
                code: KeyCode::Char('x'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => break,
            
            _ => (),
        }
    }

    //disabling raw mode
    disable_raw_mode().unwrap();
}
