fn main() {
    strcuta::PrefixMap::load("resources/波音リツ連続音Ver1.5.1/prefix.map");
    strcuta::OtoIni::explore("resources");
    println!("Hello, world!");
}
