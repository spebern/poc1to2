#[macro_use]
extern crate clap;
extern crate systemstat;

use systemstat::{System, Platform};
use std::fs::File;

struct PlotFile {
    account_id: u64,
    start_nonce: u64,
    nonce_count: u64,
    stagger: u64,
    optimized: bool,
}

fn parse_file_name(file_name: String) -> Option<PlotFile> {
    let splits: Vec<_> = file_name.split("_").collect();
    if splits.len() < 4 {
        println!("plot has invalid file name: {}", file_name);
        return None;
    }


    let account_id = splits[0].parse::<u64>().ok();
    if account_id == None {
        println!("failed to parse account id");
        return None
    }

    let start_nonce = splits[1].parse::<u64>().ok();
    if account_id == None {
        println!("failed to parse start nonce");
        return None
    }

    let nonce_count = splits[2].parse::<u64>().ok();
    if nonce_count == None {
        println!("failed to parse nonce count");
        return None
    }

    let stagger = splits[3].parse::<u64>().ok();
    if stagger == None {
        println!("failed to parse stagger");
        return None
    }

    return Some(PlotFile{
        account_id: account_id.unwrap(),
        start_nonce: start_nonce.unwrap(),
        nonce_count: nonce_count.unwrap(),
        stagger: stagger.unwrap(),
        optimized: nonce_count == stagger,
    });
}

fn main() {

    // COMMAND LINE PROCESSING

    let matches = clap_app!(myapp =>
        (version: "0.0.1")
        (author: "PoC Consortium <bots@cryptoguru.org>")
        (about: "Burst PoC1 to PoC2 converter")
        (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
        (@arg INPUT: +required "Sets the input file to use")
        (@arg debug: -d ... "Sets the level of debugging information")
        (@subcommand test =>
            (about: "controls testing features")
            (version: "1.3")
            (author: "Someone E. <someone_else@other.com>")
            (@arg verbose: -v --verbose "Print test information verbosely")
        )
    ).get_matches();
   // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let input = matches.value_of("INPUT").unwrap();
    println!("Using input file: {}", input);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }

    // FREE SYSTEM MEMORY
    
    let sys = System::new();

    match sys.memory() {
        Ok(mem) => println!("\nMemory: {} used / {} ({} bytes) total ({:?})", mem.total - mem.free, mem.total, mem.total.as_usize(), mem.platform_memory),
        Err(x) => println!("\nMemory: error: {}", x)
    }
    
    // READ FILE TO MEMORY

    let f = File::open(input).expect("file not found");
    
}

