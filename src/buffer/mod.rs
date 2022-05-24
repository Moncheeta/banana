use ropey::Rope;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use crossterm::terminal::size;

use tui::symbols::line;
use tui::widgets::Widget;

pub mod action;
pub mod history;

#[derive(Clone)]
pub struct Buffer {
    pub path: Option<PathBuf>,   // the location of the buffer in storage
    pub data: Arc<RwLock<Rope>>, // the contents of the buffer
    cursor: (usize, usize),      // the position of the cursor .0 is lines and .1 is index
    view: usize,                 // where the buffer starts to be displayed in the terminal
    history: history::History,
}

impl Buffer {
    pub fn new(path: Option<PathBuf>) -> Result<Buffer, String> {
        let data: Arc<RwLock<Rope>>;
        if let Some(ref existing_file) = path {
            match File::open(existing_file) {
                Ok(file) => {
                    data = Arc::new(RwLock::new(
                        Rope::from_reader(BufReader::new(file)).unwrap(),
                    ))
                }
                Err(_) => {
                    return Err(format!(
                        "Unable to open file: {}",
                        existing_file.as_path().display()
                    ))
                }
            };
        } else {
            data = Arc::new(RwLock::new(Rope::new()));
        }
        Ok(Buffer {
            path: {
                if let Some(new_path) = path {
                    Some(new_path)
                } else {
                    None
                }
            },
            data,
            cursor: (0, 0),
            view: 0,
            history: history::History::new(),
        })
    }

    // saves the buffer to disk
    pub fn save(&mut self) {
        let file: File;
        if let Some(path) = &self.path {
            // create the path if the file doesn't exist
            file = File::create(path)
                .expect(format!("Unable to create or open {}", path.as_path().display()).as_str());
        } else {
            // TODO: add prompt module and ask for a file name relative to the workspace path
            file = File::create("/home/moncheeta/code/banana/test/test.txt")
                .expect("Unable to save file");
        }
        self.data
            .write()
            .unwrap()
            .write_to(BufWriter::new(file))
            .expect("Unable to save file");
    }
    // validates if moving view to line to possible
    fn validate_view_move(&self, line: usize) -> bool {
        if line
            >= self.data.read().unwrap().len_lines()
                - size().expect("Unable to get the height of the terminal").1 as usize
        {
            false
        } else {
            true
        }
    }
    pub fn move_view_to(&mut self, line: usize) {
        if self.validate_view_move(line) {
            self.view = line;
        }
    }
    pub fn move_view_up(&mut self, amount: Option<usize>) {
        let amount = if let Some(new_amount) = amount {
            new_amount
        } else {
            1
        };
        self.move_view_to(self.view - amount);
    }
    pub fn move_view_down(&mut self, amount: Option<usize>) {
        let amount = if let Some(new_amount) = amount {
            new_amount
        } else {
            1
        };
        self.move_view_to(self.view + amount);
    }
    // returns true if valid
    fn validate_cursor_move(&self, new_pos: (usize, usize)) -> bool {
        if self.data.read().unwrap().len_lines() <= new_pos.0 {
            false
        } else if self.data.read().unwrap().line(new_pos.0).len_chars() <= new_pos.1 {
            false
        } else {
            true
        }
    }
    pub fn move_cursor_to(&mut self, line: usize, index: usize) {
        if self.validate_cursor_move((line, index)) {
            self.cursor.0 = line;
            self.cursor.1 = index;
        }
    }
    pub fn move_cursor_up(&mut self, amount: Option<usize>) {
        let amount = if let Some(new_amount) = amount {
            new_amount
        } else {
            1
        };
        self.move_cursor_to(self.cursor.0 - amount, self.cursor.1);
    }
    pub fn move_cursor_down(&mut self, amount: Option<usize>) {
        let amount = if let Some(new_amount) = amount {
            new_amount
        } else {
            1
        };
        self.move_cursor_to(self.cursor.0 + amount, self.cursor.1);
    }
    pub fn move_cursor_left(&mut self, amount: Option<usize>) {
        let amount = if let Some(new_amount) = amount {
            new_amount
        } else {
            1
        };
        self.move_cursor_to(self.cursor.0, self.cursor.1 - amount);
    }
    pub fn move_cursor_right(&mut self, amount: Option<usize>) {
        let amount = if let Some(new_amount) = amount {
            new_amount
        } else {
            1
        };
        self.move_cursor_to(self.cursor.0, self.cursor.1 + amount);
    }
}

impl Widget for Buffer {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let symbols = line::ROUNDED;
        let data_size_len = self.data.read().unwrap().len_lines(); // the number of lines inside of the buffer
        let mut index = 0; // the index of the buffer
        for line in 0..area.height {
            if line == 0 {
                for x in 0..area.width {
                    if x == 0 {
                        buf.get_mut(area.x + x, area.y + line)
                            .set_symbol(symbols.top_left);
                        continue;
                    } else if x == area.width - 1 {
                        buf.get_mut(area.x + x, area.y + line)
                            .set_symbol(symbols.top_right);
                        continue;
                    }
                    buf.get_mut(area.x + x, area.y + line)
                        .set_symbol(symbols.horizontal);
                }
                continue;
            } else if line == area.height - 1 {
                for x in 0..area.width {
                    if x == 0 {
                        buf.get_mut(area.x + x, area.y + line)
                            .set_symbol(symbols.bottom_left);
                        continue;
                    } else if x == area.width - 1 {
                        buf.get_mut(area.x + x, area.y + line)
                            .set_symbol(symbols.bottom_right);
                        continue;
                    }
                    buf.get_mut(area.x + x, area.y + line)
                        .set_symbol(symbols.horizontal);
                }
                continue;
            }
            buf.get_mut(area.x + index, area.y + line)
                .set_symbol(symbols.vertical);
            index += 1;
            buf.get_mut(area.x + area.width - 1, area.y + line)
                .set_symbol(symbols.vertical);
            if (line - 1) as usize >= data_size_len {
                // when the buffer ends
                // return is not called so that the rest of the border would be drawn
                index = 0;
                continue;
            } else if line >= area.height - 1 {
                // when we reach the end of the terminal
                return;
            }
            let data_size_width = self
                .data
                .read()
                .unwrap()
                .line((line - 1) as usize + self.view)
                .len_chars();
            for character in self
                .data
                .read()
                .unwrap()
                .line((line - 1) as usize + self.view)
                .chars()
            {
                if (index - 1) as usize >= data_size_width {
                    // when the index is past the width of the line inside of the buffer
                    break;
                } else if index >= area.width - 1 {
                    // when the index is past the width of the terminal
                    break;
                } else if character == '\n' {
                    break;
                }
                buf.get_mut(area.x + index, area.y + line)
                    .set_char(character);
                index += 1;
            }
            index = 0;
        }
    }
}
