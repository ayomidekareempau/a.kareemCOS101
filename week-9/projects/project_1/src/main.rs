use std::io::Write;
use std::io::Read;
use std::fs::OpenOptions;

fn main() {
    let mut file = std::fs::File::create("data.csv")
        .expect("Couldn't create file!");

    let headers = "Lager, Stout, Non-Alcoholic \n";
    let column_1 = "33 Export, Legend, Maltina\n";
    let column_2 = "Desperados, Turbo King, Amstel Malta\n";
    let column_3 = "Goldberg, Williams, Malta Gold\n";
    let column_4 = "Gulder, , Fayrouz\n";
    let column_5 = "Heineken, , \n";
    let column_6 = "Star, , \n";


    file.write_all(headers.as_bytes()).expect("write failed!");
    file.write_all(column_1.as_bytes()).expect("write failed!");
    file.write_all(column_2.as_bytes()).expect("write failed!");
    file.write_all(column_3.as_bytes()).expect("write failed!");
    file.write_all(column_4.as_bytes()).expect("write failed!");
    file.write_all(column_5.as_bytes()).expect("write failed!");
    file.write_all(column_6.as_bytes()).expect("write failed!");




}