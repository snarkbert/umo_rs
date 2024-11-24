use umo_rs::config::Config;

fn main() {
    test();
}

fn test() {
    let mut foo = Config::new();
    foo.verbose = true;
    foo.use_7z = true;
    foo.save("config.json");

    let bar = Config::load("config.json");
    println!("Config: {:?}", bar);
}