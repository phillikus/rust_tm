mod state;
mod tape;

fn main() {
    println!("Hello, world!");
    let state = state::State { state_type: state::StateType::Start, id: 1 };
    let tape = tape::Tape { alphabet: "$#".chars().collect(), head_position: 0, tape: "$|||#".chars().collect() };

    let s : String = tape.tape.iter().collect();
    println!("{0}", s);
}