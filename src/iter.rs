pub struct Splitted {}

impl Splitted {
  pub fn next_str<'a>(iterator: &mut impl Iterator<Item=&'a str>) -> &'a str {
    iterator.next().unwrap().trim()
  }

  pub fn next_f64<'a>(iterator: &mut impl Iterator<Item=&'a str>) -> f64 {
    Self::next_str(iterator).parse().unwrap()
  }
}
