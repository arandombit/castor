use serde_json::{self, Value};

use std::collections::HashMap;

use actions::Action;
use io::{read_file, save_to_file};
use structs::{Bookmark, Reading, Readings};
use views::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct OldReading {
  pub id: u32,
  pub deleted: bool,
  pub finished: bool,
  pub title: String,
  pub author: String,
  pub bookmarks: Vec<Bookmark>,
  pub pages: u32
}

type OldReadings = HashMap<u32, OldReading>;

fn map_data(data: &Value) -> Readings {
  let parsed: OldReadings = serde_json::from_str(&data.to_string()).unwrap();
  let mut transformed: Readings = HashMap::new();
  for (_, reading) in &parsed {
    let new_reading = Reading {
      id: reading.id,
      title: reading.title.clone(),
      author: reading.author.clone(),
      pages: reading.pages,
      deleted: reading.deleted,
      finished: reading.finished,
      bookmarks: reading.bookmarks.clone(),
      previous: vec![]
    };
    transformed.insert(reading.id, new_reading);
  }
  transformed
}

pub fn reducer(state: &Readings, action: &Action) -> Readings {
  let mut new_state = state.clone();
  match *action {
    Action::LoadData => {
      println!("Loading data...");
      let file = read_file().unwrap();
      let data: Value = serde_json::from_str(&file).unwrap();
      new_state = serde_json::from_str(&data.to_string()).unwrap();
      // new_state = map_data(&data);
    },
    Action::AddReading(ref reading) => {
      println!("Adding reading...");
      let id = new_state.len() as u32;
      let (title, author, pages) = reading;
      new_state.insert(
        id, Reading::new(id, title.to_string(), author.to_string(), *pages)
      );
    },
    Action::EditReading(ref reading_id, ref prop, ref edit) => {
      if let Some(reading) = new_state.get_mut(reading_id) {
        match prop.as_str() {
          "author" => reading.author = edit.to_string(),
          "title" => reading.title = edit.to_string(),
          "pages" => reading.pages = edit.parse::<u32>().unwrap(),
          _ => println!("reading Property Does Not Exist")
        }
      }
    },
    Action::FinishReading(ref reading_id) => {
      if let Some(reading) = new_state.get_mut(reading_id) {
        reading.finished = true;
      } else {
        println!("reading does not exist");
      }
    },
    Action::AddBookmark(ref reading_id, ref page) => {
      if let Some(reading) = new_state.get_mut(reading_id) {
        let last = reading.bookmarks.last().unwrap().clone();
        if last.page < *page && *page <= reading.pages {
          reading.bookmarks.push(Bookmark::new(*page));
          if *page == reading.pages { reading.finished = true; }
          print_reading(reading);
        } else {
          println!("Did you go back in time?");
        }
      } else {
        println!("reading does not exist");
      }
    },
    Action::ShowBookmarks(ref reading_id) => {
      if let Some(reading) = new_state.get(reading_id) {
        print_bookmarks(reading);
      } else {
        println!("reading does not exist");
      }
    },
    Action::UndoBookmark(ref reading_id) => {
      if let Some(reading) = new_state.get_mut(reading_id) {
        reading.bookmarks.pop();
      } else {
        println!("reading does not exist");
      }
    },
    Action::RemoveReading(ref reading_id) => {
      if let Some(reading) = new_state.get_mut(reading_id) {
        reading.deleted = true;
        println!("Reading removed");
      } else {
        println!("reading ID does not exist");
      }
    },
    Action::ResetReading(ref reading_id) => {
      if let Some(reading) = new_state.get_mut(reading_id) {
        if reading.bookmarks.len() == 1 {
          println!("You're already at page zero.");
        } else {
          reading.previous.push(reading.bookmarks.clone());
          reading.bookmarks = vec![Bookmark::new(0)];
          println!("Reading bookmarks reset");
        }
      } else {
        println!("Reading ID does not exist");
      }
    },
    Action::ReadRandom => {
      print_random(&new_state);
    },
    Action::SaveData => {
      println!("Saving readings...");
      save_to_file(&new_state).unwrap();
    },
    Action::ShowFilter(ref filter) => {
      show_filter(filter, &new_state);
    },
    Action::SearchKeyword(ref query) => {
      println!("Searching for {:?}...", query);
      let mut results = vec![];
      for reading in new_state.values() {
        let title = reading.title.to_lowercase();
        let author = reading.author.to_lowercase();
        if (title.contains(query) || author.contains(query)) && !reading.deleted {
          results.push(reading);
        }
      }
      for reading in results.iter() {
        print_reading(&reading);
      }
    },
    Action::CurrentlyReading(ref time_ago) => { 
      print_current_books(&new_state, *time_ago);
    },
    Action::GetStats(ref time_ago) => {
      print_stats(&new_state, *time_ago);
    },
    Action::ShowAverage(ref days) => {
      print_average(&new_state, *days);
    },
    Action::FindByID(ref reading_id) => {
      if let Some(reading) = new_state.get(reading_id) {
        print_reading(reading);
      } else {
        println!("reading with ID of '{:?}' does not exist", reading_id);
      }
    },
    Action::ShowFinished => {
      print_completion(&new_state, true);
    },
    Action::ShowOldestRead => {
      if new_state.len() > 0 {
        print_oldest(&new_state);
      } else {
        println!("You haven't added any readings yet");
      }
    },
    Action::ShowUnfinished => {
      print_completion(&new_state, false);
    },
    _ => ()
  }
  new_state
}
