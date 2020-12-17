use crate::{
  io::{
    IO
  }
};
use std::{
    collections::{
      HashMap,
    },
    io::{
      BufRead,
    },
    path::{
      Path,
    },
};

pub struct PrefixMap {
  entries: HashMap<String, String>
}

impl PrefixMap {
  pub fn new() -> PrefixMap {
    PrefixMap{
      entries : HashMap::new()
    }
  }

  pub fn load(path: impl AsRef<Path>) -> PrefixMap {
    let map_reader = IO::shift_jis_reader(path);

    let mut entries = HashMap::new();
    for line in map_reader.lines() {
      let line = line.unwrap();
      let (key, raw_key) = line.split_once('\t').unwrap();
      entries.insert(key.to_string(), raw_key.to_string());
    }
    PrefixMap {
      entries: entries
    }
  }
}
