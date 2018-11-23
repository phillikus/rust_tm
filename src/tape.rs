pub struct Tape<'a> {
    pub alphabet: &'a str,
    pub head_position: i32,
    pub tape: &'a str
}

impl<'a> Tape<'a> {
    pub fn write(&mut self, character: char) {
        if (self.head_position < 1 || !self.alphabet.contains(character)) {
            return;
        }

        let mut new_tape = String::with_capacity(self.tape.len());
        let mut chars = self.tape.chars();

        for i in 0..self.tape.len() {
            if (i == self.head_position as usize) {
                new_tape.push(character);
            } else {
                new_tape.push(chars.nth(i).unwrap());
            }
        }

        self.tape = new_tape.to_string();
    }

    // pub fn init(mut self, alphabet: &str, word: &str) -> () {
    //     let default_chars = "$#".to_string();
    //     self.alphabet = alphabet.to_string();
    //     self.alphabet += &default_chars;

    //     for mut character in word.chars() { 
    //         if (self.alphabet.contains(character)) {
    //             self.tape.push(character);
    //         }
    //     }

    //     self.tape.push('#');
    // }
}