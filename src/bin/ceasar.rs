extern crate getopts;
use getopts::Options;
use std::env;

use std::io;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn ceasar_cypher(inp: &Vec<u8>, out: &mut Vec<u8>, key: u8) {
}

fn do_cypher(inp_filename: &str, out_filename: &str) -> Result<(), io::Error> {
    let mut inp = Vec::new();
    let mut out = Vec::new();
    let key = 5;

    {
        let mut f = try!(File::open(&inp_filename));
        try!(f.read_to_end(& mut inp));
    }
    ceasar_cypher(&inp, &mut out, key);
    {
        let mut f = try!(File::create(&out_filename));
        try!(f.write_all(& out));
    }

    Ok(())
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "", "set output file name", "NAME");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let output = matches.opt_str("o");
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    match output {
        Some(output_str) => {
            match do_cypher(&input, &output_str) {
                Ok(m) => { println!("OK!!!") }
                Err(f) => { panic!(f.to_string()) }
            }
        }
        None => { panic!("Output file not provided") }
    }
}
