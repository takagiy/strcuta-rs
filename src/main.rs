fn main() {
    let wav = strcuta::Wav::open("resources/波音リツ連続音Ver1.5.1/通常/_ああいあうえあ.wav");
    for amp in wav.iter() {
      print!("{};", amp);
    }
    strcuta::PrefixMap::open("resources/波音リツ連続音Ver1.5.1/prefix.map");
    strcuta::OtoIni::explore("resources");
    println!("Hello, world!");
}
