use smallvec::{smallvec, SmallVec};
use std::path::PathBuf;

use tui::{layout::Layout, widgets::Widget};

use crate::buffer::Buffer;

#[derive(Clone)]
pub struct Workspace {
    pub buffers: Vec<Buffer>,              // all the buffers in the workspace
    pub opened_bufs: SmallVec<[usize; 1]>, // the currently opened buffers
    pub buffer_focus: usize,               // the buffer that the user is currently editing
    pub path: Option<PathBuf>,             // the path of the workspace
}

impl Workspace {
    pub fn new(path: Option<PathBuf>) -> Workspace {
        Workspace {
            buffers: vec![Buffer::new(path.clone()).expect("Unable to open file")],
            opened_bufs: smallvec![0],
            buffer_focus: 0,
            path,
        }
    }
    pub fn add(&mut self, path: PathBuf) -> Result<(), String> {
        let buffer = Buffer::new(Some(path));
        if let Ok(..) = buffer {
            self.buffers.push(buffer.unwrap()); // should never fail
            Ok(())
        } else {
            return Err(buffer.err().unwrap());
        }
    }

    // to add a Buffer object
    pub fn add_buf(&mut self, buf: Buffer) {
        self.buffers.push(buf);
        self.buffer_focus = self.buffers.len();
    }

    // to remove the buffer at index
    pub fn remove_buf(&mut self, index: usize) {
        self.buffers.remove(index);
        // to remove all opened views of that buffer
        for opened_buf in 0..self.opened_bufs.len() {
            if opened_buf == index {
                self.opened_bufs.remove(opened_buf);
            }
        }
        if self.buffer_focus == index {
            self.buffer_focus -= 1;
        }
    }

    // to go to the next buffer
    pub fn next_buf(&mut self) {
        if self.opened_bufs[self.buffer_focus] + 1 > self.buffers.len() {
            self.buffer_focus = 0; // if current_buf is the last buf it will go the beggining
        } else {
            self.buffer_focus += 1;
        }
    }
    pub fn prev_buf(&mut self) {
        // the conversion is needed so that that usize doesn't go below 0
        if self.opened_bufs[self.buffer_focus] as isize - 1 < 0 {
            self.buffer_focus = self.buffers.len(); // if current_buf is the first buf it will go to the end
        } else {
            self.buffer_focus -= 1;
        }
    }
    // reference to the buffer that is being edited
    pub fn cur_buf(&self) -> &Buffer {
        &self.buffers[self.opened_bufs[0]]
    }
}

impl Widget for Workspace {
    // Renders all the buffers that are currently open
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {}
}
