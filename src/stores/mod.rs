use std::collections::HashMap;

pub mod readings;

use actions::Action;
use structs::Readings;

pub struct Store<T: Clone, U> {
  state: T,
  listeners: Vec<fn(&T)>,
  reducer: fn(&T, U) -> T
}

#[derive(Clone, Debug)]
pub struct State {
  pub readings: Readings
}

impl<T: Clone, U> Store<T, U> {
  pub fn create_store(reducer: fn(&T, U) -> T, initial_state: T) -> Store<T, U> {
    Store {
      state: initial_state,
      listeners: Vec::new(),
      reducer
    }
  }
  pub fn subscribe(&mut self, listener: fn(&T)) -> &mut Store<T, U> {
    self.listeners.push(listener);
    self
  }
  pub fn get_state(&self) -> &T {
    &self.state
  }
  pub fn dispatch(&mut self, action: U) {
    self.state = (self.reducer)(&self.state, action);
    for listener in self.listeners.iter() {
      listener(&self.state)
    }
  }
}

impl State {
  pub fn new() -> State {
    State {
      readings: HashMap::new()
    }
  }
}

pub fn root_reducer(state: &State, action: Action) -> State {
  State {
    readings: readings::reducer(&state.readings, &action)
  }
}
