use std::fs::{self, File};
use std::io::Result;
use std::io::prelude::*;

use structs::Readings;

pub fn read_file() -> Result<String> {
  let data = json!({});
  let mut buffer = String::new();
  let mut file = match File::open("data.json") {
    Ok(file) => file,
    Err(_) => {
      println!("Creating data.json file...");
      File::create("data.json")?;
      fs::write("data.json", data.to_string())?;
      File::open("data.json")?
    }
  };

  file.read_to_string(&mut buffer)?;
  Ok(buffer)
}

pub fn save_to_file(readings: &Readings) -> Result<()> {
  let data = json!(readings);
  fs::write("data.json", data.to_string())?;
  Ok(())
}
