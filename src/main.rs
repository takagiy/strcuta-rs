fn main() {
    let wav = strcuta::Wav::open("resources/波音リツ連続音Ver1.5.1/通常/_ああいあうえあ.wav");
    for amp in wav.iter() {
      print!("{};", amp);
    }
    let wav = wav.as_part();
    wav.save("resources/test.wav");
    wav.save("resources/test2.wav");
    println!();
    let frq = strcuta::Frq::open("resources/波音リツ連続音Ver1.5.1/通常/_ああいあうえあ_wav.frq");
    for (frq, amp) in frq.iter() {
      print!("({},{})", frq, amp);
    }
    println!("frq chunk id {}", frq.header().chunk_id.as_ref().unwrap());
    println!("frq sampling interval {}", frq.header().sampling_interval);
    println!("frq key frequency {}", frq.header().key_frequency);
    println!("frq comment {}", frq.header().comment.as_ref().unwrap());
    println!("frq len {}", frq.header().len);
    strcuta::PrefixMap::open("resources/波音リツ連続音Ver1.5.1/prefix.map");
    strcuta::OtoIni::explore("resources");
    println!("Hello, world!");
}
