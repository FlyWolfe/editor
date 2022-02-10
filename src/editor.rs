
use crossterm::style::Stylize;

use crate::file::{self, File, FileType};
use crate::console::{Console, Key, KeyType, KeyModifier};
use crate::line::Line;

pub struct Editor {
    file: File,
    console: Console,
    lines: Vec<Line>,
    current_line: usize,
    line_index: usize,
}

impl Editor {
    pub fn new(path: &str, name: &str, extension: &str, file_type: FileType) -> Self {
        let new_file = File::new(path, name, extension, file_type);
        let new_console = Console::new(crossterm::style::Color::White);
        let mut new_lines = new_file.read();
        if (new_lines.len() <= 0) {
            new_lines.push(Line {text: "".to_string(), length: 0});
        }
        Self { file: new_file, console: new_console, lines: new_lines, current_line: 0, line_index: 0 }
    }
    
    pub fn start(&mut self) {
        self.console.start().ok();
        self.run();
    }
    
    pub fn run(&mut self) {
        loop {
            let key: Key = self.console.get_current_key();
            let mut pressedKey = true;
            match key.key_type {
                KeyType::Escape =>  {
                    self.console.close().ok();
                    break;
                },
                KeyType::Enter =>  {
                    self.lines.push(Line {text: "".to_string(), length: 0});
                    self.current_line += 1;
                    self.line_index = 0;
                },
                KeyType::Left =>  {
                    self.console.move_left();
                },
                KeyType::Right =>  {
                    self.console.move_right();
                },
                KeyType::Up =>  {
                    self.console.move_up();
                },
                KeyType::Down =>  {
                    self.console.move_down();
                },
                KeyType::Backspace =>  {
                    if (self.lines[self.current_line].length == 0) {
                        // Do nothing
                    }
                    else if (self.line_index == 0) {
                        // Do nothing
                    }
                    else {
                        self.lines[self.current_line].text.pop();
                        self.lines[self.current_line].length -= 1;
                        self.line_index -= 1;
                    }
                },
                KeyType::Character =>  {
                    self.lines[self.current_line].text.push(key.value);
                    self.lines[self.current_line].length += 1;
                    self.line_index += 1;
                },
                _ => {pressedKey = false},
            }
            
            //if (pressedKey) {
                self.console.refresh(self.lines.clone());
            //}
        }
    }
}