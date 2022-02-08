
use crate::file::{File, FileType, self};
use crate::console::Console;

pub struct Editor {
    file: File,
    console: Console,
}

impl Editor {
    pub fn new(path: String, name: String, extension: String, file_type: FileType) -> Self {
        let new_file = File::new(path, name, extension, file_type);
        let new_console = Console {};
        Self { file: new_file, console: new_console }
    }
    
    pub fn Run() {
        
    }
}