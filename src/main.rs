mod state;
mod tape;
mod direction;

fn main() {
    let state = state::State { state_type: state::StateType::Start, id: 1 };
    let mut tape = tape::Tape::new("$#", "$|||#");

    let s : String = tape.tape.iter().collect();

    tape.move_head(direction::Direction::Right);
    tape.write('#');
    tape.move_head(direction::Direction::Right);
    tape.write('#');


    println!("{0}", tape.to_string());
}