use crate::{
  io::{
    IO,
  },
  iter::{
    Splitted,
  },
  key::{
    Key,
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
    hash::{
      Hash,
    },
    borrow::{
      Borrow,
    },
    ops::{
      Index,
    }
};
use getset::{
  Getters
};
use lazy_static::{
  lazy_static,
};

#[derive(Getters, Debug)]
pub struct PrefixMap {
  #[get = "pub"]
  entries: HashMap<Key, Fixes>
}

#[derive(Getters, Debug)]
pub struct Fixes {
  #[get = "pub"]
  prefix: String,
  #[get = "pub"]
  suffix: String,
}

lazy_static! {
  static ref NO_FIXES: Fixes = Fixes {
    prefix: "".to_string(),
    suffix: "".to_string(),
  };
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
          Key::from(Splitted::next_str(items)),
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

  pub fn get<Q>(&self, key: &Q) -> &Fixes where Key: Borrow<Q>, Q: Eq + Hash {
    self.entries.get(key).unwrap_or(&NO_FIXES)
  }
}

impl Fixes {
  pub fn apply(&self, mora: &str) -> String {
    let mut result = self.prefix.to_string();
    result.push_str(mora);
    result.push_str(&self.suffix);
    result
  }
}

impl<Q> Index<&'_ Q> for PrefixMap where Key: Borrow<Q>, Q: Eq + Hash {
  type Output = Fixes;

  fn index(&self, key: &Q) -> &Self::Output {
    self.get(key)
  }
}
