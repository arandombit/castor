use chrono::prelude::*;
use chrono::Duration;
use colored::*;
use rand::Rng;

use structs::{Bookmark, Reading, Readings};

const DIVIDER: &'static str = "-------------------------------";

fn print_bookmark(bookmark: &Bookmark) {
  let Bookmark { page, date } = bookmark;
  println!("Page: {:?} - {}", page, date.format("%b %e %a %T %Y"));
}

pub fn print_bookmarks(Reading { bookmarks, .. }: &Reading) {
  for bookmark in bookmarks {
    print_bookmark(&bookmark);
  }
}

pub fn print_reading(reading: &Reading) {
  let Reading { id, title, author, pages, bookmarks, .. } = reading;
  let reading_id = id.to_string();
  let started = bookmarks.first().unwrap();
  let bookmark = bookmarks.last().unwrap();
  let progress = bookmark.page as f64 / *pages as f64 * 100.0;
  let now = Local::now();
  let last_read = now.signed_duration_since(bookmark.date);
  println!("{}\nID: ..... {}", DIVIDER.bright_red(), reading_id.yellow());
  println!("Title: .. {}", title.red());
  println!("Author: . {}", author);
  println!("Pages: .. {:?}", pages);
  println!("Started:  {}", started.date.format("%a %b %e %T %Y"));
  println!("Progress: {:.5}{}", progress.to_string().green(), "%".green());
  println!("Bookmark: Page {:?} - {}", bookmark.page, bookmark.date.format("%a %b %e %T %Y"));
  println!("Days Ago: {}\n{}", last_read.num_days().to_string().blue(), DIVIDER.bright_red());
}

pub fn print_reading_simple(reading: &Reading) {
  let Reading { id, title, author, .. } = reading;
  println!("{}: {} by {}", id.to_string().yellow(), title.red(), author);
}

pub fn print_readings(readings: &Readings) {
  println!("----------- Library -----------");
  let mut ids: Vec<u32> = readings.keys().copied().collect();
  ids.sort();
  for id in ids {
    let reading = &readings[&id];
    if !reading.deleted {
      print_reading(reading);
    }
  }
}

pub fn print_currently_reading(readings: &Readings) {
  println!("----------- Library -----------");
  for (_, reading) in readings {
    let bookmark = reading.bookmarks.last().unwrap();
    if bookmark.page > 0 {
      print_reading(&reading);
    }
  }
}

pub fn print_unread(readings: &Readings) {
  println!("----------- Library -----------");
  for (_, reading) in readings {
    let bookmark = reading.bookmarks.last().unwrap();
    if bookmark.page == 0 {
      print_reading(&reading);
    }
  }
}

pub fn print_random(readings: &Readings) {
  let mut currently_reading: Vec<&u32> = vec![];
  let mut rng = rand::thread_rng();
  for (_, reading) in readings {
    let Reading { bookmarks, deleted, finished, id, .. } = reading;
    let bookmark = bookmarks.last().unwrap();
    let page = bookmark.page as u32;
    if !finished && !deleted && page != (0 as u32) {
      currently_reading.push(id);
    }
  }
  let random_id = rng.gen_range(0, currently_reading.len() - 1);
  if let Some(reading) = readings.get(&(random_id as u32)) {
    if !reading.deleted {
      print_reading(reading);
    }
  }
}

pub fn print_completion(readings: &Readings, completion: bool) {
  let mut sum = 0;
  println!("----------- Library -----------");
  for (_, reading) in readings {
    let bookmark = reading.bookmarks.last().unwrap();
    if !reading.deleted && reading.finished == completion && bookmark.page > 0 {
      sum += 1;
      print_reading(&reading);
    }
  }
  println!("\nBooks: {}", sum);
}

pub fn print_oldest(readings: &Readings) {
  let mut oldest_read = readings[&0].bookmarks.last().unwrap().date.timestamp();
  let mut to_read = readings[&0].clone();
  for (_, book) in readings {
    let bookmark = book.bookmarks.last().unwrap();
    let timestamp = bookmark.date.timestamp();
    if timestamp < oldest_read && !book.finished && !book.deleted {
      oldest_read = timestamp;
      to_read = book.clone();
    }
  }
  print_reading(&to_read);
}

pub fn flatten_bookmarks(bookmarks: &Vec<Bookmark>) -> Vec<u32> {
  let mut pages = vec![];
  let created = bookmarks.first().unwrap();

  let mut previous = created;

  for bookmark in bookmarks {
    let Bookmark { page, date } = bookmark;
    if *page == 0 { continue; }
    let pages_read = page - previous.page;
    let gap = date.signed_duration_since(previous.date).num_days();

    if gap == 0 && pages.len() > 0 {
      let last = pages.last_mut().unwrap();
      *last += pages_read;
    } else {
      pages.push(pages_read);
    }
    previous = bookmark;
  }

  pages
}

pub fn print_average(readings: &Readings, days: u32) {
  // let now = Local::now();
  // let bookmark = bookmarks.last().unwrap();
  // let created = bookmarks.first().unwrap();
  // let pages = pages;
  // let pages_left = pages - bookmark.page;
  // let cd = created.date;
  // let duration = now.signed_duration_since(cd);
  // let daily_average = bookmark.page as f64 / duration.num_days() as f64;
  // let days_to_completion = pages_left as f64 / daily_average;
  // let days_duration = Duration::days(days_to_completion as i64);
  // let completion = now.checked_add_signed(days_duration).unwrap();
  // let finish_date = completion.format("%a %b %e %Y").to_string();
  // println!("{:?}", duration.num_days());
  // println!("Pages Left: ......... {}", pages_left);
  // println!("Daily Average: ...... {:.2}", daily_average);
  // println!("Days To Completion: . {:.2}", days_to_completion);
  // println!("Average Completion: . {}", finish_date);

  // iterate through each reading's bookmarks until x date
  // add number of pages for each reading's bookmarks that go up until x date
  // then divide by number of days to get average
}

pub fn print_ema(Reading { pages, bookmarks, .. }: &Reading) {
  let now = Local::now();
  let created = bookmarks.first().unwrap();
  let days = now.signed_duration_since(created.date);

  if days.num_days() < 2 { return println!("Insufficient Data"); }

  let mut emas: Vec<f64> = vec![];
  let k = 2.0 / (days.num_days() as f64 + 1.0);

  for pages_read in flatten_bookmarks(bookmarks) {
    if let Some(last_ema) = emas.clone().last() {
      emas.push(k * pages_read as f64 + (1.0 - k) * last_ema);
    } else {
      emas.push(k * pages_read as f64);
    }
  }

  let Bookmark { page, .. } = bookmarks.last().unwrap();
  let days_to_completion = (pages - page) as f64 / emas.last().unwrap();
  let days_duration = Duration::days(days_to_completion as i64);
  let completion = now.checked_add_signed(days_duration).unwrap();
  let finish_date = completion.format("%a %b %e %Y").to_string();
  println!("Estimated Completion: {}", finish_date.green());
}

pub fn show_filter(filter: &str, readings: &Readings) {
  match filter {
    "all" => print_readings(&readings),
    // "reading" => print_currently_reading(&readings),
    "unread" => print_unread(&readings),
    _ => println!("Not a valid filter keyword. Must use one of [all|reading|unread]")
  }
}

pub fn print_stats(readings: &Readings, time_ago: i64) {
  let mut total_read = 0;
  for (_, reading) in readings {
    let mut index = reading.bookmarks.len() - 1;
    let mut pages_read = 0;
    let bookmarks = &reading.bookmarks;
    while index > 0 {
      let r = &bookmarks[index];
      let t = r.date.timestamp();
      index -= 1;
      if t > time_ago {
        pages_read += r.page - bookmarks[index].page;
      }
    }
    total_read += pages_read;
    let last_read = bookmarks.last().unwrap().date;
    if last_read.timestamp() > time_ago && pages_read > 0 {
      print_reading(reading);
      println!("Pages Read: ......... {:?}", pages_read);
      if reading.finished {
        println!("{}", "********** Finished! **********".red());
      } else {
        print_ema(reading);
      }
    }
  }
  println!("{}\n\nTotal read: {:?}", DIVIDER.bright_red(), total_read);
}

pub fn print_current_books(readings: &Readings, time_ago: i64) {
  let mut output = vec![];
  for (_, reading) in readings {
    let mut index = reading.bookmarks.len() - 1;
    let mut pages_read = 0;
    let bookmarks = &reading.bookmarks;
    while index > 0 {
      let r = &bookmarks[index];
      let t = r.date.timestamp();
      index -= 1;
      if t > time_ago {
        pages_read += r.page - bookmarks[index].page;
      }
    }
    let bookmarks = &reading.bookmarks;
    let last_read = bookmarks.last().unwrap().date;
    if last_read.timestamp() > time_ago && pages_read > 0 {
      output.push(reading.clone());
      // print_reading_simple(reading);
    }
  }
  // println!("{:?}", output);
  output.sort_by(|a, b|
    a.title.to_lowercase().cmp(&b.title.to_lowercase())
  );
  for reading in output {
    print_reading_simple(&reading);
  }
  // println!("{:?}", output);
}
