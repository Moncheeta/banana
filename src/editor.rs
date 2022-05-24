use std::sync::{Arc, RwLock};

use std::path::PathBuf;
use std::thread;

use crate::ui::Ui;
use crate::workspace::Workspace;

pub struct Editor {
    pub workspaces: Vec<Workspace>,
    pub current_workspace: usize,
}

impl Editor {
    pub fn new(path: Option<PathBuf>) -> Editor {
        Editor {
            workspaces: vec![Workspace::new(path)],
            current_workspace: 0,
        }
    }
}

pub fn main_loop() {
    let editor = Arc::new(RwLock::new(Editor::new(None)));
    let ui_thread = thread::spawn(move || {
        let mut ui = Ui::new(editor.clone());
        ui.draw_loop();
    });
    ui_thread.join().unwrap();
}
