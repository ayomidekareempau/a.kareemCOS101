use std::io::*;
use std::fs;

fn main() {


    let mut file = fs::File::create("data.csv").expect("Couldn't create file!");
    let mut opened_file = fs::File::open("data.csv").expect("Couldnt open!");
    let mut contents = String::new();


    let headers = "Student Name, Matric.Number, Deparment, Level \n";
    let students:Vec<(String, String, String, String)> = vec![
        ("Oluchi Mordi,".to_string(), " ACC10211111,".to_string(), " Accounting,".to_string(), " 300\n".to_string()),
        ("Adams Aliyu,".to_string(), " ECO10110101,".to_string(), " Economics,".to_string(), " 100\n".to_string()),
        ("Sharia Bolade,".to_string(), "CSC10328828,".to_string(), " Computer,".to_string(), " 200\n".to_string()),
        ("Adekunle Gold,".to_string(), " EEE11020202,".to_string(), " Electrical,".to_string(), " 200\n ".to_string()),
        ("Blanca Edemoh,".to_string(), " MEE10202001,".to_string(), " Mechanical,".to_string()," 100\n".to_string())
    ];

    file.write_all(headers.as_bytes()).expect("couldn't write!");

    for (name, matric, department, level) in &students{
        file.write_all(name.as_bytes()).expect("couldn't write!");
        file.write_all(matric.as_bytes()).expect("couldn't write!");
        file.write_all(department.as_bytes()).expect("couldn't write!");
        file.write_all(level.as_bytes()).expect("couldn't write!");
    }

    opened_file.read_to_string(&mut contents).expect("couldnt read files");
    print!("{}", contents);

}
