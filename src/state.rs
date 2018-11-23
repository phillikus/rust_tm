pub enum StateType {
    Start,
    Empty,
    Final
}

pub struct State {
    pub state_type: StateType,
    pub id: i32
}