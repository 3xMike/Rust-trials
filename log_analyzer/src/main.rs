extern crate clap;
use std::{collections::HashMap, path::PathBuf};

use clap::{App, Arg, ArgMatches};
use log_analyzer::reader::MetricType;

fn config_parse() -> ArgMatches<'static> {
    App::new("Log analyzer")
        .version("1.0")
        .author("Mikhailov M. <mikhailov.mm@phystech.edu>")
        .about("Usefull app to quick analyze logs")
        .arg(
            Arg::with_name("log")
                .required(true)
                .short("f")
                .long("log")
                .takes_value(true)
                .number_of_values(1)
                .value_name("FILENAME")
                .help("Log to analyze"),
        )
        .arg(
            Arg::with_name("metrics")
                .required(true)
                .short("m")
                .long("metrics")
                .takes_value(true)
                .min_values(1)
                .help("Metrics to find in the log and include it to output table")
                .value_name("METRICS"),
        )
        .get_matches()
}

fn format_result(result: HashMap<String, Vec<MetricType>>) {
    for (metric, values) in result {
        print!("{}| ", metric);
        for value in values {
            print!("{} ", value.to_string());
        }
        println!();
    }
}

fn main() {
    //    "banshee_0.log"
    let matches = config_parse();
    if let Some(filename) = matches.value_of("log") {
        if let Some(metrics) = matches.values_of("metrics") {
            format_result(
                log_analyzer::reader::read(PathBuf::from(filename), metrics)
            );
        }
    }
}
