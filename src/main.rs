use strcuta::SampleRange;

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
    let oto_ini = strcuta::OtoIni::explore("resources/波音リツ連続音Ver1.5.1");
    println!("oto ini {:?}", oto_ini);
    let frq = strcuta::Frq::open("resources/波音リツ連続音Ver1.5.1/通常/_ああいあうえあ_wav.frq", 44100);
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
    let mut map_entries = prefix_map.entries().iter().collect::<Vec<_>>();
    map_entries.sort_by(|l, r| l.0.cmp(&r.0));
    for (key, fixes)  in map_entries {
      println!("prefix map {:?} -> {:?} {:?}", key, fixes.prefix(), fixes.suffix());
    }
    let oto = &oto_ini.entries()["u た↓"];
    println!("pre {:?}",oto.pre());
    println!("pre usize {:?}",oto.pre().to_usize_range(
            strcuta::Wav::open(oto.source_wav()).header().sample_rate));
    let voice = strcuta::Voice::new(oto);
    println!("voice pre len {:?}", voice.pre().wav().len());
    let con = voice.con();
    con.wav().save("resources/con.wav");
    let vow = voice.vow();
    vow.wav().save("resources/vow.wav");
    println!("Hello, world!");
}
