use std::io::{stdout, Stdout};
use std::sync::{Arc, RwLock, Weak};
use std::thread::sleep;
use std::time::Duration;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    terminal::Terminal,
};

use crate::editor::Editor;

pub struct Ui {
    editor: Weak<RwLock<Editor>>,
    term: Terminal<CrosstermBackend<Stdout>>,
    layout: Layout,
}

impl Ui {
    pub fn new(editor: Arc<RwLock<Editor>>) -> Ui {
        Ui {
            editor: Arc::downgrade(&editor),
            term: Terminal::new(CrosstermBackend::new(stdout()))
                .expect("Unable to access terminal"),
            layout: Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(6), Constraint::Length(3)].as_ref()),
        }
    }
    // draws ui to the screen
    pub fn draw(&mut self) {
        self.term
            .draw(|frame| {
                let layout = self.layout.split(frame.size());
                frame.render_widget(
                    self.editor.upgrade().unwrap().read().unwrap().workspaces[self
                        .editor
                        .upgrade()
                        .unwrap()
                        .read()
                        .unwrap()
                        .current_workspace]
                        .clone(),
                    layout[0],
                );
            })
            .expect("Unable to draw to the terminal");
    }

    // The draw loop is called when you start the ui. It's supposed to run on a seperate thread.
    pub fn draw_loop(&mut self) {
        enable_raw_mode().expect("Unable to enter raw mode");
        execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)
            .expect("Unable to enter alternate screen");
        loop {
            self.draw();
            sleep(Duration::from_secs(5));
            disable_raw_mode().expect("Unable to leave raw mode");
            execute!(
                self.term.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )
            .expect("Unable to leave alternate screen");
            self.term.show_cursor().expect("Unable to display cursor");
            break;
        }
    }
}
