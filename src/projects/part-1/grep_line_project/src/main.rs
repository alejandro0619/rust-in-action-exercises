use regex::Regex;
use clap::{App, Arg};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn _grep_line_using_clap(){
    let args = App::new("Grep-lite")
    .version("0.1")
    .about("Searches for patterns")
    .arg(Arg::with_name("pattern")
     .help("The patter to search for")
     .takes_value(true)
     .required(true))
     .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let quote = "Every face, every shop, badroom windows, public-house,
and dark square is a picture is a picture feverishly turned--in search of what? It is the same with books. What do we seek through million of pages?";

    for (i, line) in quote.lines().enumerate() {
        let contains_substring = re.find(line);
        match contains_substring {
            Some(_) => println!("Line: {}, Text:{} ", i, line),
            None => (),
        }
    }
}

fn grep_line_reding_file(){
    let file = File::open("readme.md").unwrap();
    let reader = BufReader::new(file);

   for line_ in reader.lines() {
       let line = line_.unwrap();
       println!("{}, {} bytes long", line, line.len());
   }
}
fn main() {
    grep_line_reding_file();
}


