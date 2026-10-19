use std::io::*;
use std::fs;

fn main() {
    let mut created_file = fs::File::create("welcome_message.txt")
        .expect("Couldn't file");
    created_file.write_all("Hello, My name is ayomide".as_bytes())
        .expect("Couldnt modify!");
    let mut file = fs::File::open("welcome_message.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
