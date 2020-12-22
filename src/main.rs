fn main() {
    let wav = strcuta::Wav::open("resources/波音リツ連続音Ver1.5.1/通常/_ああいあうえあ.wav");
    println!("wav len {:?}", wav.len());
    for amp in wav.iter() {
      print!("{:?};", amp);
    }
    println!();
    let wav = wav.as_part();
    wav.save("resources/test.wav");
    wav.save("resources/test2.wav");
    wav.cut(0..100000).save("resources/test3.wav");
    println!("wav part len {:?}", wav.len());
    let oto_ini = strcuta::OtoIni::explore("resources");
    println!("oto ini {:?}", oto_ini);
    let frq = strcuta::Frq::open("resources/波音リツ連続音Ver1.5.1/通常/_ああいあうえあ_wav.frq");
    println!("frq len {:?}", frq.len());
    println!("{:?}", wav.header());
    for (frq, amp) in frq.iter() {
      print!("({:?},{:?})", frq, amp);
    }
    println!();
    println!("frq chunk id {:?}", frq.header().chunk_id.as_ref().unwrap());
    println!("frq sampling interval {:?}", frq.header().sampling_interval);
    println!("frq key frequency {:?}", frq.header().key_frequency);
    println!("frq comment {:?}", frq.header().comment.as_ref().unwrap());
    println!("frq len {:?}", frq.header().len);
    let prefix_map = strcuta::PrefixMap::open("resources/波音リツ連続音Ver1.5.1/prefix.map");
    for (key, fixes)  in prefix_map.entries() {
      println!("prefix map {:?} -> {:?} {:?}", key, fixes.prefix(), fixes.suffix());
    }
    let voice = strcuta::Voice::new(oto_ini.entries().values().next().unwrap());
    println!("voice pre len {:?}", voice.pre().wav().len());
    println!("Hello, world!");
}
