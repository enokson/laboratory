use crate::reporter::Reporter;
use bincode::{serialize, deserialize};
use serde::{Deserialize, Serialize};

pub struct State {
    state: Vec<u8>
}
impl State {
    pub fn new() -> State {
        State { state: vec![] }
    }
    pub fn get<'a, T>(&'a self) -> Result<T, String>
        where
            T: Deserialize<'a>
    {
        let value: Result<T, String> = match deserialize(&self.state) {
            Ok(value) => Ok(value),
            Err(_error) => Err(format!("Could not seserialize state."))
        };
        value
    }
    pub fn set<T: Serialize>(& mut self, state: T) -> Result<(), String> {
        let state = match serialize(&state) {
            Ok(vec) => Ok(vec),
            Err(_error) => Err(format!("Could not serialize state."))
        }?;
        self.state = state;
        Ok(())
    }
    pub fn get_raw_state(&self) -> Vec<u8> {
        self.state.to_vec()
    }
    pub fn set_raw_state(&mut self, vec: Vec<u8>) {
        self.state = vec;
    }
}
