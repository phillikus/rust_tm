mod state;
mod tape;

fn main() {
    let state = state::State { state_type: state::StateType::Start, id: 1 };
    let mut tape = tape::Tape::new("$#", "$|||#");

    let s : String = tape.tape.iter().collect();

    tape.move_head(1);
    tape.write('#');
    tape.move_head(5);
    tape.write('#');


    println!("{0}", tape.to_string());
}