use std::collections::HashMap;

use chrono::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bookmark {
  pub page: u32,
  pub date: DateTime<Local>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Reading {
  pub id: u32,
  pub deleted: bool,
  pub finished: bool,
  pub title: String,
  pub author: String,
  pub bookmarks: Vec<Bookmark>,
  pub previous: Vec<Vec<Bookmark>>,
  pub pages: u32
}

impl Bookmark {
  pub fn new(page: u32) -> Self {
    Self { page, date: Local::now() }
  }
}

impl Reading {
  pub fn new(id: u32, title: String, author: String, pages: u32) -> Self {
    Self {
      id,
      title,
      author,
      pages,
      deleted: false,
      finished: false,
      bookmarks: vec![Bookmark::new(0)],
      previous: vec![]
    }
  }
}

pub type Readings = HashMap<u32, Reading>;
pub type Error = &'static str;
