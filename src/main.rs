#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate crushtool;
extern crate simple_logger;

use std::io::{self, Read};

use clap::{App, Arg, ArgGroup};
use crushtool::{CrushMap, decode_crushmap};

fn main() {
    let matches = App::new("crushtool")
        .version(crate_version!())
        .arg(Arg::with_name("verbose")
            .short("v")
            .multiple(true)
            .help("Sets the level of debugging information"))
        .arg(Arg::with_name("decompile")
            .short("d")
            .help("Decompile a crush map")
            .conflicts_with("compile"))
        .arg(Arg::with_name("compile")
            .short("c")
            .help("Compile a crush map")
            .conflicts_with("decompile"))
        .group(ArgGroup::with_name("mode")
            .required(true)
            .args(&["compile", "decompile"]))
        .get_matches();
    let log_level = match matches.occurrences_of("verbose") {
        0 => log::LogLevel::Warn,
        1 => log::LogLevel::Info,
        2 => log::LogLevel::Debug,
        3 | _ => log::LogLevel::Trace,
    };

    simple_logger::init_with_level(log_level).unwrap();

    let mut buffer: Vec<u8> = vec![];
    match io::stdin().read_to_end(&mut buffer) {
        Ok(_) => trace!("Read input from STDIN"),
        Err(e) => trace!("Failed to read STDIN: {:?}", e),
    };

    let input: &[u8] = &buffer.as_slice();

    if matches.is_present("decompile") {
        let result: CrushMap = match decode_crushmap(&input) {
            Ok(r) => r,
            _ => panic!("There was a problem parsing the crushmap"),
        };
        println!("{:?}", result);
    } else if matches.is_present("compile") {
        println!("Coming soon!");
    }
}
