use crate::line::Line;
pub enum FileType {
    Rust,
    Custom,
    Other
}

pub struct File {
    path: String,
    name: String,
    extension: String,
    file_type: FileType,
}

impl File {
    pub fn new(path: &str, name: &str, extension: &str, file_type: FileType) -> Self {
        Self { path: path.to_string(), name: name.to_string(), extension: extension.to_string(), file_type: file_type }
    }
    
    pub fn open() {
        
    }
    
    pub fn read(&self) -> Vec<Line> {
        let mut full_document: Vec<Line> = Vec::new();
        
        // TODO
        
        return full_document;
    }
    
    pub fn write(text: String) {
        
    }
    
    pub fn close() {
        
    }

}