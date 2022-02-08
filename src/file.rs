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
    pub fn new(path: String, name: String, extension: String, file_type: FileType) -> Self {
        Self { path: path, name: name, extension: extension, file_type: file_type }
    }
    
    pub fn Open() {
        
    }
    
    pub fn Read() -> String {
        let full_document = "";
        
        // TODO
        
        return full_document.to_string();
    }
    
    pub fn Write(text: String) {
        
    }
    
    pub fn Close() {
        
    }

}