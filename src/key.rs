#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Key(u32, KeyName);

pub use KeyName::*;

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum KeyName {
  C,
  CSharp,
  D,
  DSharp,
  E,
  F,
  FSharp,
  G,
  GSharp,
  A,
  ASharp,
  B,
}

pub const C1: Key = Key(1, C);
pub const CS1: Key = Key(1, CSharp);
pub const D1: Key = Key(1, D);
pub const DS1: Key = Key(1, DSharp);
pub const E1: Key = Key(1, E);
pub const F1: Key = Key(1, F);
pub const FS1: Key = Key(1, FSharp);
pub const G1: Key = Key(1, G);
pub const GS1: Key = Key(1, GSharp);
pub const A1: Key = Key(1, A);
pub const AS1: Key = Key(1, ASharp);
pub const B1: Key = Key(1, B);

pub const C2: Key = Key(2, C);
pub const CS2: Key = Key(2, CSharp);
pub const D2: Key = Key(2, D);
pub const DS2: Key = Key(2, DSharp);
pub const E2: Key = Key(2, E);
pub const F2: Key = Key(2, F);
pub const FS2: Key = Key(2, FSharp);
pub const G2: Key = Key(2, G);
pub const GS2: Key = Key(2, GSharp);
pub const A2: Key = Key(2, A);
pub const AS2: Key = Key(2, ASharp);
pub const B2: Key = Key(2, B);

pub const C3: Key = Key(3, C);
pub const CS3: Key = Key(3, CSharp);
pub const D3: Key = Key(3, D);
pub const DS3: Key = Key(3, DSharp);
pub const E3: Key = Key(3, E);
pub const F3: Key = Key(3, F);
pub const FS3: Key = Key(3, FSharp);
pub const G3: Key = Key(3, G);
pub const GS3: Key = Key(3, GSharp);
pub const A3: Key = Key(3, A);
pub const AS3: Key = Key(3, ASharp);
pub const B3: Key = Key(3, B);

pub const C4: Key = Key(4, C);
pub const CS4: Key = Key(4, CSharp);
pub const D4: Key = Key(4, D);
pub const DS4: Key = Key(4, DSharp);
pub const E4: Key = Key(4, E);
pub const F4: Key = Key(4, F);
pub const FS4: Key = Key(4, FSharp);
pub const G4: Key = Key(4, G);
pub const GS4: Key = Key(4, GSharp);
pub const A4: Key = Key(4, A);
pub const AS4: Key = Key(4, ASharp);
pub const B4: Key = Key(4, B);

pub const C5: Key = Key(5, C);
pub const CS5: Key = Key(5, CSharp);
pub const D5: Key = Key(5, D);
pub const DS5: Key = Key(5, DSharp);
pub const E5: Key = Key(5, E);
pub const F5: Key = Key(5, F);
pub const FS5: Key = Key(5, FSharp);
pub const G5: Key = Key(5, G);
pub const GS5: Key = Key(5, GSharp);
pub const A5: Key = Key(5, A);
pub const AS5: Key = Key(5, ASharp);
pub const B5: Key = Key(5, B);

pub const C6: Key = Key(6, C);
pub const CS6: Key = Key(6, CSharp);
pub const D6: Key = Key(6, D);
pub const DS6: Key = Key(6, DSharp);
pub const E6: Key = Key(6, E);
pub const F6: Key = Key(6, F);
pub const FS6: Key = Key(6, FSharp);
pub const G6: Key = Key(6, G);
pub const GS6: Key = Key(6, GSharp);
pub const A6: Key = Key(6, A);
pub const AS6: Key = Key(6, ASharp);
pub const B6: Key = Key(6, B);

pub const C7: Key = Key(7, C);
pub const CS7: Key = Key(7, CSharp);
pub const D7: Key = Key(7, D);
pub const DS7: Key = Key(7, DSharp);
pub const E7: Key = Key(7, E);
pub const F7: Key = Key(7, F);
pub const FS7: Key = Key(7, FSharp);
pub const G7: Key = Key(7, G);
pub const GS7: Key = Key(7, GSharp);
pub const A7: Key = Key(7, A);
pub const AS7: Key = Key(7, ASharp);
pub const B7: Key = Key(7, B);
