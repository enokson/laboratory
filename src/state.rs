use bincode::{serialize, deserialize};
use serde::{Deserialize, Serialize};

pub struct State {
    state: Vec<u8>
}
impl State {
    pub fn new() -> State {
        State { state: vec![] }
    }
    pub fn get_state<'a, T>(&'a self) -> T
        where
            T: Deserialize<'a>
    {
        deserialize(&self.state).expect("Could not deserialize state.")
        // from_str(&self.state).expect("Could not convert from string")
    }
    pub fn set_state<T: Serialize>(& mut self, state: T) {
        // self.state = to_string(&state).expect("Could not convert to String.");
        self.state = serialize(&state).expect("Could not serialize state.");
    }
    pub fn get_raw_state(&self) -> Vec<u8> {
        self.state.to_vec()
    }
    pub fn set_raw_state(&mut self, vec: Vec<u8>) {
        self.state = vec;
    }
}
