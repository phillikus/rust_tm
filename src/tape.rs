pub struct Tape {
    pub alphabet: Vec<char>,
    pub head_position: usize,
    pub tape: Vec<char>
}

impl Tape {
    pub fn write(&mut self, character: char) {
        if self.head_position < 1 || !self.alphabet.contains(&character) {
            return;
        }

        self.tape[self.head_position] = character;
    }
}