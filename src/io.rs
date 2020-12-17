use std::{
  io::{
    BufReader,
    BufRead,
  },
  fs::{
    File
  },
  path::{
    Path,
  }
};
use encoding_rs::{
  SHIFT_JIS,
};
use encoding_rs_io::{
  DecodeReaderBytes,
  DecodeReaderBytesBuilder,
};

pub struct IO {
}
impl IO {
  pub fn shift_jis_reader(path: impl AsRef<Path>) -> impl BufRead {
    let file = File::open(path.as_ref()).unwrap();
    let decoder = DecodeReaderBytesBuilder::new()
        .encoding(Some(SHIFT_JIS))
        .build(file);
    BufReader::new(decoder)
  }
}
