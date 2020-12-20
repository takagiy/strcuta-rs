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
  },
};
use encoding_rs::{
  SHIFT_JIS,
};
use encoding_rs_io::{
  DecodeReaderBytesBuilder,
};
use byteorder::{
  ReadBytesExt
};

pub struct IO {}

pub type StringOrBytes = Result<String, Vec<u8>>;

pub trait StringOrBytesExt {
  fn new(source: Vec<u8>) -> StringOrBytes;
}

pub trait BinaryRead: ReadBytesExt {
  fn read_string_from_chunk(&mut self, chunk_size: usize) -> StringOrBytes {
    let mut buffer = vec![0u8; chunk_size];
    self.read(&mut buffer).unwrap();
    StringOrBytes::new(buffer)
  }
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

impl StringOrBytesExt for StringOrBytes {
  fn new(mut source: Vec<u8>) -> StringOrBytes {
    let valid_length =
        source.iter().position(|c| *c == b'\0').unwrap_or(source.len());
    source.truncate(valid_length);
    String::from_utf8(source).map_err(|e| e.into_bytes())
  }
}

impl<R: ReadBytesExt> BinaryRead for R {}
