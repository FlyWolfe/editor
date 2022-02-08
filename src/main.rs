extern crate crossterm;

mod editor;
mod file;
mod console;

use std::{io::{self, stdout, Write, Read}, env};

use crossterm::{
    cursor::{self, MoveLeft, MoveRight, MoveTo, MoveDown, MoveUp}, event::{self, Event, KeyEvent, KeyCode}, execute, queue,
    style::{self, Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, Stylize},
    terminal::{self, ScrollUp, SetSize, size, enable_raw_mode, disable_raw_mode},
    ExecutableCommand, Result,
};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1);
    let name = args.get(2);
    let extension = args.get(3);
    let file_type = args.get(4);
    
    
    
    
    
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    
    let (cols, rows) = size()?;
    
    enable_raw_mode().expect("Could not turn on Raw mode");
    
    
    execute!(
        stdout,
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
    )?;
    
    loop {
        if let Event::Key(event) = event::read().expect("Failed to read character") {
            match event {
                KeyEvent {
                    code: KeyCode::Esc,
                    modifiers: event::KeyModifiers::NONE,
                } => break,
                KeyEvent {
                    code: KeyCode::Backspace,
                    modifiers: event::KeyModifiers::NONE,
                } => {
                    execute!(
                        stdout,
                        MoveLeft(1),
                        Print(' '),
                        MoveLeft(1),
                    )?;
                },
                KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: event::KeyModifiers::NONE,
                } => {
                    execute!(
                        stdout,
                        Print('\n'),
                    )?;
                },
                KeyEvent {
                    code: KeyCode::Left,
                    modifiers: event::KeyModifiers::NONE,
                } => {
                    execute!(
                        stdout,
                        MoveLeft(1),
                    )?;
                },
                KeyEvent {
                    code: KeyCode::Right,
                    modifiers: event::KeyModifiers::NONE,
                } => {
                    execute!(
                        stdout,
                        MoveRight(1),
                    )?;
                },
                KeyEvent {
                    code: KeyCode::Up,
                    modifiers: event::KeyModifiers::NONE,
                } => {
                    execute!(
                        stdout,
                        MoveUp(1),
                    )?;
                },
                KeyEvent {
                    code: KeyCode::Down,
                    modifiers: event::KeyModifiers::NONE,
                } => {
                    execute!(
                        stdout,
                        MoveDown(1),
                    )?;
                },
                KeyEvent {
                    code: KeyCode::Char(c),
                    modifiers: event::KeyModifiers::NONE,
                } => {
                    execute!(
                        stdout,
                        Print(c),
                    )?;
                },
                KeyEvent {
                    code: KeyCode::Char(c),
                    modifiers: event::KeyModifiers::SHIFT,
                } => {
                    execute!(
                        stdout,
                        Print(c.to_ascii_uppercase()),
                    )?;
                },
                _ => {}
            }
        };
    }
    
    disable_raw_mode().expect("Unable to disable raw mode");
    
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    
    stdout.flush()?;
    Ok(())
}
