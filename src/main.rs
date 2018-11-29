mod state;
mod tape;
mod direction;
mod transition;
mod tm;

fn main() {
    let mut tm : tm::TM = subtract("$|||#||");
    tm.process(true);
}

fn increment(word: &str) -> tm::TM {
    let mut tape = tape::Tape::new("$|#", word);
    let states = vec![
        state::State::new('0', state::StateType::Start),
        state::State::new('1', state::StateType::Empty),
        state::State::new('f', state::StateType::Final)
    ];

    let transitions = vec![
        transition::Transition::new('0', '$', '1', '$', direction::Direction::Right),
        transition::Transition::new('1', '|', '1', '|', direction::Direction::Right),
        transition::Transition::new('1', '#', 'f', '|', direction::Direction::None)
    ];

    tm::TM::new(states, transitions, tape)
}

fn decrement(word: &str) -> tm::TM {
    let tape = tape::Tape::new("$|#", word);
    let states = vec![
        state::State::new('0', state::StateType::Start),
        state::State::new('1', state::StateType::Empty),
        state::State::new('2', state::StateType::Empty),
        state::State::new('f', state::StateType::Final)
    ];

    let transitions = vec![
        transition::Transition::new('0', '$', '1', '$', direction::Direction::Right),
        transition::Transition::new('1', '|', '1', '|', direction::Direction::Right),
        transition::Transition::new('1', '#', '2', '#', direction::Direction::Left),
        transition::Transition::new('2', '$', 'f', '$', direction::Direction::None),
        transition::Transition::new('2', '|', 'f', '#', direction::Direction::None)
    ];

    tm::TM::new(states, transitions, tape)
}

fn add(word: &str) -> tm::TM {
    let tape = tape::Tape::new("$|&#", word);
    let states = vec![
        state::State::new('0', state::StateType::Start),
        state::State::new('1', state::StateType::Empty),
        state::State::new('2', state::StateType::Empty),
        state::State::new('3', state::StateType::Empty),
        state::State::new('4', state::StateType::Empty),
        state::State::new('f', state::StateType::Final)
    ];

    let transitions = vec![
        transition::Transition::new('0', '$', '1', '$', direction::Direction::Right),
        transition::Transition::new('1', '#', 'f', '#', direction::Direction::None),
        transition::Transition::new('1', '|', '1', '|', direction::Direction::Right),
        transition::Transition::new('1', '&', '2', '|', direction::Direction::Right),
        transition::Transition::new('2', '|', '2', '|', direction::Direction::Right),
        transition::Transition::new('2', '#', '3', '#', direction::Direction::Left),
        transition::Transition::new('3', '|', '4', '#', direction::Direction::Left),
        transition::Transition::new('4', '|', '4', '|', direction::Direction::Left),
        transition::Transition::new('4', '$', 'f', '$', direction::Direction::None)
    ];

    tm::TM::new(states, transitions, tape)
}

fn subtract(word: &str) -> tm::TM {
    let tape = tape::Tape::new("$|#", word);
    let states = vec![
        state::State::new('0', state::StateType::Start),
        state::State::new('1', state::StateType::Empty),
        state::State::new('2', state::StateType::Empty),
        state::State::new('3', state::StateType::Empty),
        state::State::new('4', state::StateType::Empty),
        state::State::new('5', state::StateType::Empty),
        state::State::new('6', state::StateType::Empty),
        state::State::new('7', state::StateType::Empty),
        state::State::new('8', state::StateType::Empty),
        state::State::new('f', state::StateType::Final)
    ];

    let transitions = vec![
        transition::Transition::new('0', '$', '0', '$', direction::Direction::Right),
        transition::Transition::new('0', '#', 'f', '#', direction::Direction::None),
        transition::Transition::new('0', '|', '1', '|', direction::Direction::Right),
        transition::Transition::new('1', '|', '1', '|', direction::Direction::Right),
        transition::Transition::new('1', '#', '2', '#', direction::Direction::Right),
        transition::Transition::new('2', '#', '2', '#', direction::Direction::Right),
        transition::Transition::new('2', '|', '3', '|', direction::Direction::Right),
        transition::Transition::new('3', '|', '4', '|', direction::Direction::Left),
        transition::Transition::new('3', '#', '6', '#', direction::Direction::Left),
        transition::Transition::new('4', '|', '5', '#', direction::Direction::Left),
        transition::Transition::new('5', '#', '5', '#', direction::Direction::Left),
        transition::Transition::new('5', '|', '2', '#', direction::Direction::Right),
        transition::Transition::new('5', '$', '2', '$', direction::Direction::Right),
        transition::Transition::new('6', '|', '7', '#', direction::Direction::Left),
        transition::Transition::new('7', '#', '7', '#', direction::Direction::Left),
        transition::Transition::new('7', '$', 'f', '$', direction::Direction::None),
        transition::Transition::new('7', '|', '8', '#', direction::Direction::Left),
        transition::Transition::new('8', '|', '8', '|', direction::Direction::Left),
        transition::Transition::new('8', '$', 'f', '$', direction::Direction::None)
    ];

    tm::TM::new(states, transitions, tape)
}