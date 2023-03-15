use std::io::{stdout, Write};
use crossterm::{execute, cursor, Result};

pub(crate) fn controls(input:String) -> Result<()> {
    // Move the cursor up by three rows
    
    if input == "^[[A" {
        execute!(stdout(), cursor::MoveUp(1))?;
    }else if input == "^[[C" {
        execute!(stdout(), cursor::MoveRight(1))?;
    }else if input == "^[[B" {
        execute!(stdout(), cursor::MoveDown(1))?;
    }else if input == "^[[D" {
        execute!(stdout(), cursor::MoveLeft(1))?;
    }

    // Print a message after moving the cursor
    //writeln!(stdout(), "Hello, world!")?;

    Ok(())
}
