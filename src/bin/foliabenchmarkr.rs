extern crate clap;
extern crate libc;

use std::str;
use std::time::{SystemTime,Duration};
use std::os;
use std::mem;
use std::fs;
use clap::App;
use folia::*;

//getrusage functions extracted from: https://github.com/anler/getrusage/
pub struct Rusage(libc::rusage);

pub type Errno = os::raw::c_int;

fn get_resource_usage(who: os::raw::c_int) -> Result<libc::rusage, Errno> {
    let mut data = unsafe { mem::uninitialized() };

    let result = unsafe { libc::getrusage(who, &mut data) };

    if result == -1 {
        Err(unsafe { *libc::__errno_location() })
    } else {
        Ok(data)
    }
}

// Memory reporting (copied from librustc source code itself)
#[cfg(unix)]
fn get_resident() -> Option<usize> {
    let field = 1;
    let contents = fs::read("/proc/self/statm").ok()?;
    let contents = String::from_utf8(contents).ok()?;
    let s = contents.split_whitespace().nth(field)?;
    let npages = s.parse::<usize>().ok()?;
    Some(npages * 4096)
}


impl Rusage {
     pub fn new() -> Result<Self, Errno> {
        get_resource_usage(libc::RUSAGE_SELF).map(Rusage)
     }

     pub fn peakrss(&self) -> i64 {
         self.0.ru_maxrss
     }
}




struct Measurement {
    begintime: SystemTime,
    duration: Option<Duration>,
    beginmem: usize,
    endmem: usize,
    beginpeak: i64,
}

impl Measurement {
    fn begin() -> Self  {
        Self {
            begintime: SystemTime::now(),
            duration: None,
            beginmem: get_resident().expect("unwrapping memory"),
            endmem: 0,
            beginpeak: Rusage::new().unwrap().peakrss()
        }
    }

    fn end(&mut self, test_id: &str, filename: &str, title: &str) {
        self.duration = self.begintime.elapsed().ok();
        self.endmem = get_resident().expect("unwrapping memory");
        let mem = self.endmem - self.beginmem;
        let rusage = Rusage::new().expect("Rusage");
        let endpeak = rusage.peakrss();
        let peakmem = endpeak - self.beginpeak;
        println!("{} - [{}] - {} - time: {}ms, mem: {}, peak mem: {}", filename, test_id, title, self.duration.unwrap().as_secs_f64() * 1000.0, mem, peakmem);
    }
}

fn test(test_id: &str, filename: &str) {
    match test_id {
        "parse" => {
            let mut m = Measurement::begin();
            let doc = Document::from_file(filename, DocumentProperties::default()).expect("loading folia document");
            m.end(test_id, filename, "Parse XML from file into full memory representation");
            doc.id(); //just to make sure the compiler doesn't optimise doc away (not sure if needed but better safe than sorry)
        },
        _ => {
            eprintln!("No such test: {}", test_id);
        }
    };
}

fn main() {
    let argmatches = App::new("foliabenchmarkr")
        .version("0.0.1")
        .author("Maarten van Gompel (proycon) <proycon@anaproy.nl>")
        .about("FoLiA benchmark")
        .arg(clap::Arg::with_name("tests")
            .help("Comma separated lists of tests to run")
            .short("t")
            .long("tests")
            .takes_value(true)
            .default_value("parse")
        )
        .arg(clap::Arg::with_name("file")
            .help("FoLiA document to parse")
            .multiple(true)
            .required(true)
        ).get_matches();

    let tests: Vec<&str> = argmatches.value_of("tests").expect("No tests specified").split_terminator(",").collect();

    for filename in argmatches.values_of("file").expect("Expected one or more files") {
        for test_id in tests.iter() {
            test(*test_id, filename)
        }
    }
}
