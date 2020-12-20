use crate::{
  io::{
    IO,
  },
  iter::{
    Splitted,
  },
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
use getset::{
  Getters
};

#[derive(Getters)]
pub struct PrefixMap {
  #[get = "pub"]
  entries: HashMap<String, Fixes>
}

#[derive(Getters)]
pub struct Fixes {
  #[get = "pub"]
  prefix: String,
  #[get = "pub"]
  suffix: String,
}

impl PrefixMap {
  pub fn new() -> PrefixMap {
    PrefixMap{
      entries : HashMap::new()
    }
  }

  pub fn open(path: impl AsRef<Path>) -> PrefixMap {
    let map_reader = IO::shift_jis_reader(path);

    let mut entries = HashMap::new();
    for line in map_reader.lines() {
      let line = line.unwrap();
      let items = &mut line.split('\t');
      entries.insert(
          Splitted::next_str(items).to_string(),
          Fixes {
            prefix: Splitted::next_str(items).to_string(),
            suffix: Splitted::next_str(items).to_string(),
          }
      );
    }
    PrefixMap {
      entries: entries
    }
  }
}
