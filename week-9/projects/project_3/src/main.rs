use std::io::*;
use std::fs;

fn main() {
    let commisioners:Vec<String> = vec![
        "Aigbogun Alambda Daudu,".to_string(), 
        "Murtala Afeez Bendu,".to_string(),
        "Okorocha Calistuc Ogbona,".to_string(),
        "Adewale Jimoh Akanbi,".to_string(),
        "Osazuwa Faith Etieye,".to_string()
    ];

    let ministry:Vec<String> = vec![
        "Internal Affairs,".to_string(),
        "Justice,".to_string(),
        "Defense,".to_string(),
        "Power &Steel,".to_string(),
        "Petroleum,".to_string()
    ];

    let zone:Vec<String> = vec![
        "South West,\n".to_string(),
        "North East,\n".to_string(),
        "South South,\n".to_string(),
        "South West,\n".to_string(),
        "South East,\n".to_string()
    ];

    let mut commisioners_file = fs::File::create("commisioners.csv")
        .expect("Couldn't create file!");

    let mut  ministry_file = fs::File::create("ministry.csv")
        .expect("Couldn't create file!");

    let mut zone_file = fs::File::create("zone.csv")
        .expect("Couldn't create file!");

    let mut master_file = fs::File::create("master_file.csv")
        .expect("Couldnt create file!");

    for i in &commisioners{
        commisioners_file.write_all(i.as_bytes())
            .expect("Couldnt write!");
    }

    for i in &ministry{
        ministry_file.write_all(i.as_bytes())
            .expect("Couldnt write!");
    }

    for i in &zone{
        zone_file.write_all(i.as_bytes())
            .expect("Couldn't write!");
    }

    master_file.write_all("Commissioner, Ministry, Zone \n".as_bytes())
                .expect("Couldnt write!");
    for i in 0..commisioners.len(){
        master_file.write_all(commisioners[i].as_bytes())
            .expect("couldnt write!");
        master_file.write_all(ministry[i].as_bytes())
            .expect("couldnt write!");
        master_file.write_all(zone[i].as_bytes())
            .expect("couldnt write!");
    }
}