use crossterm::{QueueableCommand, execute, cursor, Result};
use std::io::{Write, stdout};


pub(crate) fn controller(user_input: &mut String) { 
    //println!("Debug: input = {}", user_input);
    let mut a = 1;
    let mut w = 1;
    let mut s = 1;
    let mut d = 1; 

    if user_input.trim() == "w" { execute!(stdout(), cursor::MoveUp(w)); w += 1;

    }else if user_input.trim() == "d" { execute!(stdout(), cursor::MoveRight(d)); d += 1;
    
    }else if user_input.trim() == "s" { execute!(stdout(), cursor::MoveDown(s)); s += 1;
    
    }else if user_input.trim() == "a" { execute!(stdout(), cursor::MoveLeft(a)); a += 1;

    }else { println!("Debug: Not detected!");}

     
    
    /*let mut stdout = stdout();
    stdout.queue(cursor::MoveTo(5,5));
 
    stdout.flush();*/
}
