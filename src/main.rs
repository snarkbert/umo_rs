use std::fs::File;
use umo_rs::config::Config;

fn main() {
    let foo = Config::new();
    //println!("config is : {:?}", foo);

    let output_file = File::create("config.json")
        .expect("Could not create output file");

    serde_json::to_writer_pretty(output_file, &foo)
        .expect("Error writing output file");
}
