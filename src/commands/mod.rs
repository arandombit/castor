use chrono::prelude::*;

use actions::Action;
use parser::Parser;
use structs::Error;

type P<'a, 'b, F> = &'a mut Parser<'b, F>;

const HELP: &str = include_str!("../assets/help.txt");

pub fn command_help() {
  print!("{HELP}");
}

pub fn add_reading(parser: P<impl FnMut(Action)>) -> Result<(), Error> {
  let title = parser.next()?;
  let author = parser.next()?;
  let pages = parser.next()?;

  let reading = (title.to_string(), author.to_string(), pages.parse::<u32>().unwrap());

  Ok((parser.dispatch)(Action::AddReading(reading)))
}

pub fn edit_reading(parser: P<impl FnMut(Action)>) -> Result<(), Error> {
  let reading = parser.next()?.parse::<u32>().unwrap();
  let prop = parser.next()?;
  let edit = parser.next()?;
  Ok((parser.dispatch)(Action::EditReading(reading, prop.to_string(), edit.to_string())))
}

pub fn currently_reading(parser: P<impl FnMut(Action)>) -> Result<(), Error> {
  let span = parser.next()?;
  let seconds_in_day = 86400;
  let current_time = Local::now();
  let time_ago = match span.parse::<u64>() {
    Ok(number) => current_time.timestamp() - seconds_in_day * number as i64,
    Err(_) => return Err("Invalid time span parameter was supplied")
  };
  Ok((parser.dispatch)(Action::CurrentlyReading(time_ago)))
}

pub fn finish_reading(parser: P<impl FnMut(Action)>) -> Result<(), Error> {
  let reading = parser.next()?.parse::<u32>().unwrap();
  Ok((parser.dispatch)(Action::FinishReading(reading)))
}

pub fn undo_bookmark(parser: P<impl FnMut(Action)>) -> Result<(), Error> {
  let reading = parser.next()?.parse::<u32>().unwrap();
  Ok((parser.dispatch)(Action::UndoBookmark(reading)))
}

pub fn remove_reading(parser: P<impl FnMut(Action)>) -> Result<(), Error> {
  let reading = parser.next()?;
  if let Ok(id) = reading.parse::<u32>() {
    Ok((parser.dispatch)(Action::RemoveReading(id)))
  } else {
    Err("Invalid ID parameter was supplied")
  }
}

pub fn reset_reading(parser: P<impl FnMut(Action)>) -> Result<(), Error> {
  let reading = parser.next()?;
  if let Ok(id) = reading.parse::<u32>() {
    Ok((parser.dispatch)(Action::ResetReading(id)))
  } else {
    Err("Invalid ID parameter was supplied")
  }
}

pub fn search_keyword(parser: P<impl FnMut(Action)>) -> Result<(), Error> {
  let reading = parser.next()?;
  Ok((parser.dispatch)(Action::SearchKeyword(reading.to_lowercase().to_string())))
}

pub fn add_bookmark(parser: P<impl FnMut(Action)>) -> Result<(), Error> {
  let id = parser.next()?.parse::<u32>().unwrap();
  let page = parser.next()?.parse::<u32>().unwrap();
  Ok((parser.dispatch)(Action::AddBookmark(id, page)))
}

pub fn show_bookmarks(parser: P<impl FnMut(Action)>) -> Result<(), Error> {
  if let Ok(id) = parser.next()?.parse::<u32>() {
    Ok((parser.dispatch)(Action::ShowBookmarks(id)))
  } else {
    Err("Parse Error: ID provided was not in integer format")
  }
}

pub fn find_reading(parser: P<impl FnMut(Action)>) -> Result<(), Error> {
  if let Ok(id) = parser.next()?.parse::<u32>() {
    Ok((parser.dispatch)(Action::FindByID(id)))
  } else {
    Err("Parse Error: ID provided was not in integer format")
  }
}

pub fn get_stats(parser: P<impl FnMut(Action)>) -> Result<(), Error> {
  let span = parser.next()?;
  let seconds_in_day = 86400;
  let current_time = Local::now();
  let time_ago = match span {
    "day" => current_time.timestamp() - seconds_in_day,
    "week" => current_time.timestamp() - seconds_in_day * 7,
    "biweek" => current_time.timestamp() - seconds_in_day * 14,
    "month" => current_time.timestamp() - seconds_in_day * 30,
    "yeartodate" => 0,
    "all" => 0,
    _ => match span.parse::<u64>() {
      Ok(number) => current_time.timestamp() - seconds_in_day * number as i64,
      Err(_) => return Err("Invalid span parameter was supplied")
    }
  };
  Ok((parser.dispatch)(Action::GetStats(time_ago)))
}

pub fn show_readings(parser: P<impl FnMut(Action)>) -> Result<(), Error> {
  let filter = parser.next()?;
  Ok((parser.dispatch)(Action::ShowFilter(filter.to_string())))
}

pub fn reading_avg(parser: P<impl FnMut(Action)>) -> Result<(), Error> {
  if let Ok(days) = parser.next()?.parse::<u32>() {
    Ok((parser.dispatch)(Action::ShowAverage(days)))
  } else {
    Err("Parse Error: Days provided was not in integer format")
  }
}
