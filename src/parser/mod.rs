use std::process;

use actions::Action;
use commands::*;

type Error = &'static str;

pub struct Parser<'a, F: FnMut(Action)> {
  pub dispatch: F,
  position: usize,
  words: Vec<&'a str>
}

impl<'a, F: FnMut(Action)> Parser<'a, F> {
  pub fn new(words: Vec<&'a str>, dispatch: F) -> Self {
    Parser { words, dispatch, position: 0 }
  }
  pub fn next(&mut self) -> Result<&'a str, Error> {
    match self.words.get(self.position) {
      Some(ref word) => {
        self.position += 1;
        Ok(word)
      },
      _ => Err("Parse Error: Command Not Recognized")
    }
  }
  fn last(&mut self) -> Result<(), Error> {
    if self.position >= self.words.len() {
      Ok(())
    } else {
      Err("Parse Error: Trailing Words")
    }
  }
  pub fn parse(&mut self) -> Result<(), Error> {
    let command = self.next()?;
    let result = match command {
      "id" | "$" => { find_reading(self).ok(); },
      "add" => { add_reading(self).ok(); },
      "avg" => { reading_avg(self).ok(); },
      "bookmark" | "!!" => { add_bookmark(self).ok(); },
      "bookmarks" => { show_bookmarks(self).ok(); },
      "current" => { currently_reading(self).ok(); },
      "edit" => { edit_reading(self).ok(); },
      "exit" => { process::exit(0) },
      "finish" => { finish_reading(self).ok(); },
      "help" => command_help(),
      "rm" => { remove_reading(self).ok(); },
      "reset" => { reset_reading(self).ok(); },
      "find" => { search_keyword(self).ok(); },
      "show" => { show_readings(self).ok(); },
      "stats" => { get_stats(self).ok(); },
      "time" => { println!("Best time for reading"); },
      "undo" => { undo_bookmark(self).ok(); },
      "random" => (self.dispatch)(Action::ReadRandom),
      "finished" => (self.dispatch)(Action::ShowFinished),
      "last" => (self.dispatch)(Action::ShowOldestRead),
      "save" | "++" => (self.dispatch)(Action::SaveData),
      "unfinished" => (self.dispatch)(Action::ShowUnfinished),
      // _ => return Err("Parse Error: Unknown Command")
      _ => { println!("Parse Error: Unknown command"); }
    };
    self.last()?;
    Ok(result)
  }
}
