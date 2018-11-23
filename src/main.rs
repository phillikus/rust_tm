mod state;
mod tape;

fn main() {
    println!("Hello, world!");
    let state = state::State { state_type: state::StateType::Start, id: 1 };
    let tape = tape::Tape { alphabet: "$#", head_position: 0, tape: "$|||#" };
    println!("{}",tape.tape);
}