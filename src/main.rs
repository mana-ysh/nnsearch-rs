extern crate clap;

use clap::{App, Arg, SubCommand};
use std::process::exit;

fn main() {
    let matches = App::new("nnsearch")
                    .about("Nearest neighbor searcher for Rust")
                    .version("0.1.0")
                    .subcommand(SubCommand::with_name("index")
                                .about("indexing objects")
                                .arg(Arg::with_name("input").help("path to input vector file"))
                                .arg(Arg::with_name("output").help("path to output file")))
                    .subcommand(SubCommand::with_name("search")
                                .about("searching from indexed objects")
                                .arg(Arg::with_name("index").help("index file"))
                                .arg(Arg::with_name("query").help("query file")))
                    .get_matches();
    if let Some(_matches) = matches.subcommand_matches("index") {
        println!("running indexing");
        exit(0);
    }

    if let Some(_matches) = matches.subcommand_matches("search") {
        println!("running search");
        exit(0);
    }
}