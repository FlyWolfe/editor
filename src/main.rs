use editor::Editor;

extern crate crossterm;
extern crate log;

mod editor;
mod file;
mod console;
mod line;

fn main() {
    /*let args: Vec<String> = env::args().collect();
    let path = args.get(1);
    let name = args.get(2);
    let extension = args.get(3);
    let file_type = args.get(4);*/
    let mut editor: Editor = Editor::new("", "", "", file::FileType::Other);
    editor.start();
}
