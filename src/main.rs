#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

extern crate chrono;
extern crate colored;
extern crate rand;

mod actions;
mod commands;
mod io;
mod parser;
mod stores;
mod structs;
mod views;

use std::io::Write;

use actions::Action;
use parser::Parser;
use stores::{Store, State, root_reducer};

fn main() {
  let mut store = Store::create_store(root_reducer, State::new());

  store.dispatch(Action::LoadData);
  store.dispatch(Action::ShowWeeklyChart);

  let stdin = std::io::stdin();
  let mut stdout = std::io::stdout();
  let mut buffer = String::new();

  loop {
    write!(stdout, "> ").ok();
    buffer.clear();
    stdout.flush().unwrap();
    stdin.read_line(&mut buffer).unwrap();

    let words: Vec<&str> = buffer.split('@').map(|x| x.trim()).collect();

    Parser::new(words, |v| store.dispatch(v)).parse().ok();
  }
}
