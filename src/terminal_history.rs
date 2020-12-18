pub struct TerminalHistory {
    history_limit: usize,
    pointer: usize,
    count: usize,
    inputs: Vec<String>
}

impl TerminalHistory {
    pub fn new(history_limit: usize) -> Self {
        TerminalHistory {
            history_limit,
            pointer: 0,
            count: 0,
            inputs: Vec::<String>::with_capacity(history_limit)
        }
    }

    pub fn prev(&mut self) -> Option<String> {
        if self.count == 0 {
            None
        } else {
            self.pointer = if self.pointer == 0 { self.count - 1 } else { self.pointer - 1 };
            let s = &self.inputs[self.pointer];
            Some(s.clone())
        }
    }

    pub fn next(&mut self) -> Option<String> {
        if self.count == 0 {
            None
        } else {
            self.pointer = if self.pointer + 1 >= self.count { 0 } else { self.pointer + 1 };
            let s = &self.inputs[self.pointer];
            Some(s.clone())
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn add(&mut self, input: String) {
        if input.trim().is_empty() || self.inputs.contains(&input) {
            return;
        }

        if self.count == self.history_limit {
            for i in 1..self.history_limit {
                self.inputs[i - 1] = self.inputs[i].clone();
            }
            self.inputs[self.count - 1] = input;
        } else {
            self.count += 1;
            self.inputs.push(input);
        }
        self.pointer = self.count;
    }
}

#[test]
fn terminal_history_tests() {
    let mut history = TerminalHistory::new(3);
    history.add("first".to_string());
    history.add("second".to_string());
    history.add("third".to_string());

    assert_eq!(history.count(), 3);

    assert_eq!(history.next().unwrap(), "first");
    assert_eq!(history.prev().unwrap(), "third");
    assert_eq!(history.prev().unwrap(), "second");
    assert_eq!(history.prev().unwrap(), "first");
    assert_eq!(history.prev().unwrap(), "third");
    assert_eq!(history.prev().unwrap(), "second");
}

#[test]
fn terminal_history_overflow_test() {
    let mut history = TerminalHistory::new(3);
    history.add("first".to_string());
    history.add("second".to_string());
    history.add("third".to_string());
    history.add("fourth".to_string());
    history.add("fifth".to_string());

    assert_eq!(history.prev().unwrap(), "fifth");
    assert_eq!(history.prev().unwrap(), "fourth");
    assert_eq!(history.prev().unwrap(), "third");
    assert_eq!(history.prev().unwrap(), "fifth");
}

#[test]
fn terminal_histore_unique_values_test() {
    let mut history = TerminalHistory::new(3);

    history.add("    ".to_string());
    history.add("".to_string());
    assert_eq!(history.inputs.is_empty(), true);

    history.add("1".to_string());
    history.add("2".to_string());
    history.add("1".to_string());
    assert_eq!(history.inputs, ["1", "2"]);


    history.add("first".to_string());
    history.add("second".to_string());
    history.add("third".to_string());
}