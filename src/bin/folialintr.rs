extern crate clap;

use std::str;
use clap::App;
use folia::*;

fn main() {
    let argmatches = App::new("folialintr")
        .version("0.0.1")
        .author("Maarten van Gompel (proycon) <proycon@anaproy.nl>")
        .about("FoLiA parser")
        .arg(clap::Arg::with_name("file")
            .help("FoLiA document to parse")
            .multiple(true)
            .required(true)
        ).get_matches();

    for filename in argmatches.values_of("file").expect("Expected one or more files") {
        match Document::from_file(filename, DocumentProperties::default()) {
            Ok(doc) => {
                match doc.xml(0) {
                    Ok(xml) => println!("{}",str::from_utf8(&xml).unwrap()),
                    Err(err) => eprintln!("{}",err)
                }
            },
            Err(err) => eprintln!("{}",err)
        }
    }
}
