// Rust crates
use std::*;
use std::io::{stdout, Write};
use unindent::unindent;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::*;
use clearscreen::*;
use ansi_escapes::*;

// Personal Crates
use crate::cwd::*;
use crate::term_prep::*;

pub(crate) fn sp_program_run() {
    // Variable initizations and declarations 
    let arg: Vec<String> = env::args().collect();
    
    // Finds current directory to set filepath for editing 
    let curd = get_current_working_dir();
    
    let mut filepath:String = curd.to_owned();
    
    let buf:&str = "/";
    let file:&String = &arg[1];
   
    // Appends to the filepath string 
    filepath.push_str(&buf);
    filepath.push_str(&file);
    

    // Takes file data and converts to a string
    let mut data = fs::read_to_string(filepath).expect("Unable to read file"); 
    let mut buffer_data = String::new();
    
    // Cleans and preps stdout() for printing
    term_prep(data);
        

    // Sets the cursor below the menu and prints file contents 
    execute!(stdout(), cursor::MoveTo(0, 1)).unwrap();

    //key detection
    loop {
        // Turns on raw mode
        enable_raw_mode().unwrap();

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
            }) => print!(""),


            // Exit without saving
            Event::Key(KeyEvent {
                code: KeyCode::Char('x'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => break,

            _ => (),
        }

        disable_raw_mode().unwrap();
    }
    clearscreen::clear().expect("failed to clear screen");

    //disabling raw mode
    disable_raw_mode().unwrap();
}

