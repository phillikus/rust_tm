mod state;
mod tape;
mod direction;
mod transition;
mod tm;

fn main() {
    let mut tm : tm::TM = increment("$||#");
    tm.process(true);

    println!("Tape after processing: {0}", tm.tape.to_string());

    let mut tm2 : tm::TM = decrement("$|||||#");
    tm2.process(true);

    println!("Tape after processing: {0}", tm2.tape.to_string());
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