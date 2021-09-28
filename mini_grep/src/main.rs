extern crate clap;
use clap::{App, Arg, ArgGroup};
use core_utils::{cat, grep, find, threadpool};

pub fn main() {
    threadpool::run();
    let matches = App::new("My core utils")
        .version("1.0")
        .author("Mikhailov M. <mikhailov.mm@phystech.edu>")
        .about("Some default config utils")
        .group(ArgGroup::with_name("commands")
            .required(true)
            .args(&["cat","grep","find", "none"])
            .multiple(false)
        )
        .arg(Arg::with_name("none")
        .short("n")
        .long("none")
        .takes_value(false)
        .help("Do nothing (I'm testing main)")
        )
        .arg(Arg::with_name("cat")
            .short("c")
            .long("cat")
            .takes_value(true)
            .help("Concatinate and print FILES")
            .value_name("FILES")
        )
        .arg(Arg::with_name("grep")
            .short("g")
            .long("grep")
            .takes_value(true)
            .help("Searching lanes in FILE with SUBSTRING")
            .number_of_values(2)
            .value_names(&["SUBSTRING", "FILE"])
        )
        .arg(Arg::with_name("find")
            .short("f")
            .long("find")
            .takes_value(true)
            .help("find file or dir in given dir")
            .max_values(2)
//            .value_names(&["FILENAME", "BASE_DIR"])
        )
        .get_matches();

// MAYBE SHOULD PASS ARGS AS ITERATOR TO
// EACH FUNCTION AND PARSE IT INSIDE 
// ISTEAD OF PARSING PARAMS IN MAIN
    if let Some(values) = matches.values_of("cat") {
        if let Err(e) = cat::exec(values) {
            panic!("{}", e);
        }
    }
    if let Some(mut values) = matches.values_of("grep") {
        if let Some(substring) = values.next() {
            if let Some(file) = values.next() {
                if let Err(e) = grep::exec(substring,file){
                    panic!("{}", e);
                }
            }
        }
    }

    if let Some(mut values) = matches.values_of("find") {
        if let Some(file) = values.next() {
            let dir = match values.next() {
                Some(dir) => dir,
                None => ".",
            };
            if let Err(e) = find::exec(file,dir){
                panic!("{}", e);
            }
        }  
    }
}
