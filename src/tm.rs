use tape;
use transition;
use direction;
use state;

pub struct TM {
    states: Vec<state::State>,
    transitions: Vec<transition::Transition>,
    pub tape: tape::Tape
} 

impl TM {
    pub fn new(states: Vec<state::State>, transitions: Vec<transition::Transition>, tape: tape::Tape) -> TM {
        TM { states, transitions, tape }
    }

    pub fn process(&mut self, verbose: bool) {
        let mut current_state: state::State = self.get_first_state();
        let mut step: i32 = 0;

        self.log_step(step, verbose);

        while current_state.state_type != state::StateType::Final {
            let current_char = self.tape.read();
            let state_id = current_state.id;

            let transition = *self.transitions.iter().clone()
            .find(|transition| transition.current_state == state_id && transition.current_char == current_char).unwrap();

            current_state = *self.states.iter().clone().find(|state| state.id == transition.new_state).unwrap();

            step += 1;
            self.tape.write(transition.new_char);
            self.tape.move_head(transition.direction);
            self.log_step(step, verbose);
        }
    }

    fn get_first_state(&self) -> state::State {
        let mut iter = self.states.iter().cloned();
        let first_state: Option<state::State> = iter.find(|state| state.state_type == state::StateType::Start);

        match first_state {
            None => panic!("No start state found."),
            Some(state) => state
        }
    }

    fn log_step(&mut self, step: i32, verbose: bool) {
        if !verbose {
            return
        }

        println!("Tape after step {0}: {1} -> Head position: {2}", step, self.tape.to_string(), self.tape.head_position);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_tm() -> TM {
        let tape = tape::Tape::new("$|#", "$||#");
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

        return TM::new(states, transitions, tape);
    }

    #[test]
    fn new_should_create_instance() {
        let tm : TM = create_tm();
        assert_eq!("$||#", tm.tape.to_string());
    }

    #[test]
    fn get_first_state_should_return_correct_state() {
        let tm : TM = create_tm();
        let first_state = tm.get_first_state();
        assert_eq!('0', first_state.id);
        assert_eq!(state::StateType::Start, first_state.state_type);
    }

    #[test]
    fn process_should_yield_finished_tm() {
        let mut tm : TM = create_tm();
        tm.process(false);
        assert_eq!("$|||", tm.tape.to_string());
    }
}