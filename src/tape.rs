use direction;
pub struct Tape {
    pub alphabet: Vec<char>,
    pub head_position: i32,
    pub tape: Vec<char>
}

impl Tape {
    pub fn new(alphabet: &str, tape: &str) -> Tape {
        Tape { alphabet: alphabet.chars().collect(), head_position: 0, tape: tape.chars().collect() }
    }

    pub fn write(&mut self, character: char) {
        if self.head_position < 1 || !self.alphabet.contains(&character) {
            return
        }

        self.tape[self.head_position as usize] = character;
    }

    pub fn read(&mut self) -> char {
        if self.head_position as usize > self.tape.len() {
            panic!("Trying to read character at invalid position: {}", self.head_position.to_string());
        }

        self.tape[self.head_position as usize]
    }

    pub fn move_head(&mut self, direction: direction::Direction) {
        match direction {
            direction::Direction::Right => { self.head_position += 1; },
            direction::Direction::Left => { self.head_position -= 1; },
            direction::Direction::None => {}
        }
        
        if self.head_position < 0 {
            self.head_position = 0;
        }

        if self.head_position >= self.tape.len() as i32 {
            self.tape.push('#');
        }
    }

    pub fn to_string(&self) -> String {
        self.tape.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_create_instance() {
        let tape = Tape::new("$#", "$|||#");
        assert_eq!("$|||#", tape.to_string())
    }

    #[test]
    fn write_should_change_value_at_valid_position() {
        let mut tape = Tape::new("$#", "$|||#");
        tape.move_head(direction::Direction::Right);
        tape.write('#');
        assert_eq!("$#||#", tape.to_string())
    }

    #[test]
    fn write_should_have_no_effect_on_tape_start() {
        let mut tape = Tape::new("$#", "$|||#");
        tape.write('#');
        assert_eq!("$|||#", tape.to_string());
    }

    #[test]
    fn move_head_past_end_should_append_hash_at_end_of_tape() {
        let mut tape = Tape::new("$#", "$|||#");
        tape.head_position = 4;
        tape.move_head(direction::Direction::Right);
        assert_eq!("$|||##", tape.to_string());
    }

    #[test]
    fn read_should_return_character_at_head_position() {
        let mut tape = Tape::new("$#", "$|||#");
        tape.head_position = 1;
        assert_eq!('|', tape.read());
    }

    #[test]
    #[should_panic]
    fn reading_past_tape_should_panic() {
        let mut tape = Tape::new("$#", "$|||#");
        tape.head_position = 5;
        tape.read();
    }
}