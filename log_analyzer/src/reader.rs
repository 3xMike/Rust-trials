#![feature(buf_read_has_data_left)]
use serde_json::{de::StrRead, Value};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug)]
pub enum MetricType {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

impl MetricType {
    pub fn to_string(&self) -> String {
        match self {
            MetricType::None => format!(""),
            MetricType::Bool(b) => format!("{}", &b),
            MetricType::Int(i) => format!("{}", &i),
            MetricType::Float(f) => format!("{}", &f),
            MetricType::String(s) => s.clone(),
        }
    }
    fn from(arg: &serde_json::Value) -> Self {
        if let Some(bool) = arg.as_bool() {
            return MetricType::Bool(bool);
        }
        if let Some(int) = arg.as_i64() {
            return MetricType::Int(int);
        }
        if let Some(float) = arg.as_f64() {
            return MetricType::Float(float);
        }
        if let Some(string) = arg.as_str() {
            return MetricType::String(string.to_string());
        }
        MetricType::None
    }
}

fn parse<'a>(
    params: &'a serde_json::Value,
    sought_metrics: &Vec<String>,
) -> Vec<(&'a str, &'a serde_json::Value)> {
    let mut result = Vec::<(&str, &serde_json::Value)>::new();
    if let Some(obj) = params.as_object() {
        for (_, category) in obj {
            if let Some(metrics) = category.as_object() {
                for (metric, value) in metrics {
                    if sought_metrics.contains(metric) {
                        result.push((metric, value));
                    }
                }
            }
        }
    }
    result
}

pub fn read(filename: PathBuf, sought_metrics: clap::Values) -> HashMap<String, Vec<MetricType>> {
    let sought_metrics: Vec<String> = sought_metrics.map(|x| x.to_string()).collect();
    let mut result = HashMap::<String, Vec<MetricType>>::new();
    for metric in &sought_metrics {
        result.insert(metric.clone(), Vec::<MetricType>::new());
    }
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    loop {
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                if let Some(pos) = line.find("metrics_exporter_log(INFO)") {
                    let u: serde_json::Value =
                        serde_json::de::from_str(&line[pos + 28..]).expect("Uncorrect json format");
                    for (metric, value) in parse(&u, &sought_metrics) {
                        if let Some(vec) = result.get_mut(metric) {
                            vec.push(MetricType::from(value));
                        }
                    }
                }
            }
            Err(e) => panic!("{}", e),
        }
        line.clear();
    }
    result
}
