use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
};

pub struct Terminal {
    buffer: Arc<Mutex<Vec<Option<String>>>>,
    stdout: io::Stdout,
}

const ANSI_PURGE: &[u8; 4] = b"\x1b[3J";
const ANSI_CURSOR_HOME: &[u8; 3] = b"\x1b[H";

impl Terminal {
    pub fn new(buffer_size: usize) -> Self {
        let mut vec = Vec::new();
        for _ in 0..buffer_size {
            vec.push(None);
        }

        Self {
            buffer: Arc::new(Mutex::new(vec)),
            stdout: io::stdout(),
        }
    }

    pub fn update_line(&self, line_number: usize, value: String) {
        let mut buffer = self.buffer.lock().unwrap();
        if line_number < buffer.len() {
            buffer[line_number] = Some(value);
        }
    }

    pub fn render(&self) {
        let mut stdout = self.stdout.lock();

        stdout.write(ANSI_PURGE).unwrap();
        stdout.write(ANSI_CURSOR_HOME).unwrap();

        let buffer = self.buffer.lock().unwrap();
        for line in buffer.iter() {
            match line {
                Some(str) => stdout.write(format!("{}\n", str).as_bytes()).unwrap(),
                None => stdout.write(String::from("\n").as_bytes()).unwrap(),
            };
        }
        stdout.flush().unwrap();
    }

    pub fn clean_term(&self) {
        let mut stdout = self.stdout.lock();
        stdout.write(ANSI_PURGE).unwrap();
        stdout.write(ANSI_CURSOR_HOME).unwrap();
        stdout.flush().unwrap();
    }
}
