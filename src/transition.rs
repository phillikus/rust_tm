mod state;
mod direction;

pub struct Transition {
    current_state: State,
    current_char: char,
    new_state: State,
    new_char: char,
    direction: Direction
}