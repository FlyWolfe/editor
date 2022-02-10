use std::{io::{self, stdout, Write, Read, Stdout}, env};

use crossterm::{
    cursor::{self, MoveLeft, MoveRight, MoveTo, MoveDown, MoveUp, MoveToRow, MoveToNextLine, MoveToColumn}, event::{self, Event, KeyEvent, KeyCode}, execute, queue,
    style::{self, Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, Stylize},
    terminal::{self, ScrollUp, SetSize, size, enable_raw_mode, disable_raw_mode},
    ExecutableCommand, Result,
};

use crate::line::Line;

pub enum KeyType {
    Escape,
    Backspace,
    Enter,
    Character,
    Left,
    Right,
    Up,
    Down,
    None,
}

pub enum KeyModifier {
    Control,
    Shift,
    Alt,
    Command,
    None
}

pub struct Key {
    pub key_type: KeyType,
    pub key_modifier: KeyModifier,
    pub value: char,
}

pub struct Console {
    color: Color,
    stdout: Stdout,
}

impl Console {
    pub fn new(color: Color) -> Console {
        Self {color: color, stdout: stdout()}
    }
    
    pub fn start(&mut self) -> Result<()> {
        execute!(self.stdout, terminal::Clear(terminal::ClearType::All))?;
        
        let (cols, rows) = size()?;
        
        enable_raw_mode().expect("Could not turn on Raw mode");
        
        execute!(
            self.stdout,
            SetForegroundColor(self.color),
            SetBackgroundColor(Color::Black),
        )?;
        Ok(())
    }
    
    pub fn close(&mut self) -> Result<()> {
        disable_raw_mode().expect("Unable to disable raw mode");
    
        execute!(self.stdout, terminal::Clear(terminal::ClearType::All))?;
        
        self.stdout.flush()?;
        Ok(())
    }
    
    pub fn get_current_key(&mut self) -> Key {
        if let Event::Key(event) = event::read().expect("Failed to read character") {
            match event {
                KeyEvent {
                    code: KeyCode::Esc,
                    modifiers: event::KeyModifiers::NONE,
                } => return Key {key_type: KeyType::Escape, key_modifier: KeyModifier::None, value: '\0'},
                KeyEvent {
                    code: KeyCode::Backspace,
                    modifiers: event::KeyModifiers::NONE,
                } => return Key {key_type: KeyType::Backspace, key_modifier: KeyModifier::None, value: '\0'},
                KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: event::KeyModifiers::NONE,
                } =>  return Key {key_type: KeyType::Enter, key_modifier: KeyModifier::None, value: '\0'},
                KeyEvent {
                    code: KeyCode::Left,
                    modifiers: event::KeyModifiers::NONE,
                } => return Key {key_type: KeyType::Left, key_modifier: KeyModifier::None, value: '\0'},
                KeyEvent {
                    code: KeyCode::Right,
                    modifiers: event::KeyModifiers::NONE,
                } => return Key {key_type: KeyType::Right, key_modifier: KeyModifier::None, value: '\0'},
                KeyEvent {
                    code: KeyCode::Up,
                    modifiers: event::KeyModifiers::NONE,
                } => return Key {key_type: KeyType::Up, key_modifier: KeyModifier::None, value: '\0'},
                KeyEvent {
                    code: KeyCode::Down,
                    modifiers: event::KeyModifiers::NONE,
                } => return Key {key_type: KeyType::Down, key_modifier: KeyModifier::None, value: '\0'},
                KeyEvent {
                    code: KeyCode::Char(c),
                    modifiers: event::KeyModifiers::NONE,
                } => return Key {key_type: KeyType::Character, key_modifier: KeyModifier::None, value: c},
                KeyEvent {
                    code: KeyCode::Char(c),
                    modifiers: event::KeyModifiers::SHIFT,
                } => return Key {key_type: KeyType::Character, key_modifier: KeyModifier::Shift, value: c},
                _ => return Key {key_type: KeyType::None, key_modifier: KeyModifier::None, value: '\0'},
            }
        };
        return Key {key_type: KeyType::None, key_modifier: KeyModifier::None, value: '\0'}
    }
    
    pub fn move_left(&mut self) -> Result<()> {
        execute!(
            self.stdout,
            MoveLeft(1),
        )?;
        Ok(())
    }
    
    pub fn move_right(&mut self) -> Result<()> {
        execute!(
            self.stdout,
            MoveRight(1),
        )?;
        Ok(())
    }
    
    pub fn move_up(&mut self) -> Result<()> {
        execute!(
            self.stdout,
            MoveUp(1),
        )?;
        Ok(())
    }
    
    pub fn move_down(&mut self) -> Result<()> {
        execute!(
            self.stdout,
            MoveDown(1),
        )?;
        Ok(())
    }
    
    pub fn print(&mut self, c: char) -> Result<()> {
        execute!(
            self.stdout,
            Print(c),
        )?;
        Ok(())
    }
    
    pub fn refresh(&mut self, lines: Vec<Line>) -> Result<()> {
        stdout().execute(terminal::Clear(terminal::ClearType::All)).ok();
        
        for line in lines {
            execute!(
                self.stdout,
                MoveToNextLine(1),
                Print(&line.text),
            )?;
            //println!("\nCurrent Line: {}", &line.text);
        }
        
        Ok(())
    }
}