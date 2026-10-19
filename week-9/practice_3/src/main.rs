use std::*;

fn main() {
    let file = fs::File::create("data.txt").expect("Couldn't create file");
    fs::remove_file("data.txt").expect("could not remove file!");
    println!("file is removed");
}