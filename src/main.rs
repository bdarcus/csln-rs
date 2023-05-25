use std::env;
use std::fs;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;

#[allow(unused_imports)]
mod bibliography;
use bibliography::InputBibliography as Bib;

#[allow(unused_imports)]
mod style;
use style::Style;

fn main() {
    // Get the command line arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Please provide style path and bibliography path as command line arguments.");
    }
    let style_path = &args[1];
    let bib_path = &args[2];

    // Parse the style file.
    let style_json = fs::read_to_string(style_path)
        .expect("Unable to read style file");
    let style: Style = if style_path.ends_with(".json") {
        serde_json::from_str(&style_json).unwrap()
    } else if style_path.ends_with(".yaml") || style_path.ends_with(".yml") {
        serde_yaml::from_str(&style_json).unwrap()
    } else {
        panic!("Unsupported file format for style file.");
    };

    // Parse the bibliography file.
    let bib_json = fs::read_to_string(bib_path)
        .expect("Unable to read bibliography file");
    let bib: Bib = if bib_path.ends_with(".json") {
        serde_json::from_str(&bib_json).unwrap()
    } else if bib_path.ends_with(".yaml") || bib_path.ends_with(".yml") {
        serde_yaml::from_str(&bib_json).unwrap()
    } else {
        panic!("Unsupported file format for bibliography file.");
    };

    // Do something with the style and bibliography data.
    println!("The name of the style is: {}", serde_json::to_string(&style.title).unwrap());
    println!("The number of entries in the bibliography is: {}", bib.len());
}