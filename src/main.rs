use std::fs::File;
use std::io::prelude::*;

fn read_file(fileuri: String) {
    println!("In file {}", fileuri);

    let mut f = File::open(fileuri).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}

fn write_file()

fn main() {
    println!("Hello, world!");
    let fileuri = "./text.txt";
    read_file(fileuri.to_string());
}
