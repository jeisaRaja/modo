mod editor;

use editor::Editor;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    Editor::new().unwrap().run();
}
