use console::{ Term, Key };

use crate::terminal_history::TerminalHistory;

pub struct Terminal {
    terminal: Term,
    prompt_msg: String,
    history: TerminalHistory,
}

impl Terminal {
    pub fn new(prompt_msg: String, history_limit: usize) -> Self {
        Terminal {
            terminal: Term::buffered_stdout(),
            prompt_msg,
            history: TerminalHistory::new(history_limit)
        }
    }

    fn write_str(&self, str: &str) {
        self.terminal.write_str(str).unwrap();
        self.terminal.flush().unwrap();        
    }

    pub fn read_line(&mut self) -> String {
        self.write_str(self.prompt_msg.as_str());

        let mut input = String::default();
        loop {
            if let Ok(key) = self.terminal.read_key() {
                match key {
                    Key::ArrowUp => {
                        if let Some(str) = self.history.prev() {
                            self.terminal.clear_chars(input.chars().count()).unwrap();
                            input = str;
                            self.write_str(input.as_str());
                        }
                    },
                    Key::ArrowDown => {
                        if let Some(str) = self.history.next() {
                            self.terminal.clear_chars(input.chars().count()).unwrap();
                            input = str;
                            self.write_str(input.as_str());
                        }
                    },
                    Key::Enter => {
                        self.terminal.write_line("").unwrap();
                        self.terminal.flush().unwrap();
                        self.history.add(input.clone());
                        break;
                    },
                    Key::Char(char) => {
                        input.push(char);
                        self.terminal.write_str(char.to_string().as_str()).unwrap();
                        self.terminal.flush().unwrap();
                    },
                    Key::Backspace => {
                        if !input.is_empty() {
                            self.terminal.clear_chars(1).unwrap();
                            input.pop();
                        }
                    }
                    _ => {}
                }
            }
        }
        input
    }

    pub fn clear(&self) {
        match self.terminal.clear_screen() {
            Err(_) => {},
            Ok(()) => {}
        }
    }
}