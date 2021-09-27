use std::collections::VecDeque;
use std::fs;
use std::io;
use std::path::PathBuf;

fn recurse_dir(dir: PathBuf) -> impl Iterator<Item = io::Result<PathBuf>> {
    let mut unvisited = VecDeque::new();
    unvisited.push_back(dir.clone());
//    let mut backup_iter: Option<fs::ReadDir> = None;
    let mut backup_iter = None.into_iter().flatten();
    let mut dir = dir;
    std::iter::from_fn(move || {   
        loop {
            match backup_iter.next() {
                None => {
                    dir = unvisited.pop_front()?;
                    match fs::read_dir(&dir) {
                        Ok(read_dir) => backup_iter = Some(read_dir).into_iter().flatten(),
                        Err(e) => return Some(Err(e)),
                    };
                }
                Some(entry) => match entry {
                    Ok(dir_entry) => match dir_entry.file_type() {
                        Ok(file_type) => {
                            if file_type.is_dir() {
                                unvisited.push_back(dir_entry.path());
                                return Some(Ok(dir_entry.path()));
                            } else if file_type.is_file() {
                                return Some(Ok(dir_entry.path()));
                            }
                        }
                        Err(e) => return Some(Err(e)),
                    },
                    Err(e) => return Some(Err(e)),
                },
            }
        }
    })
}

fn search_in<'a>(
    filename: &'a str,
    path: PathBuf,
) -> impl Iterator<Item = io::Result<PathBuf>> + 'a {
    recurse_dir(path).filter(move |x| match x {
        Err(_) => true,
        Ok(path) => {
            path.file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                == filename
        }
    })
}
/*
fn parse_args(mut _args: Vec<String>) -> Result<(String, PathBuf), String> {
    let path;
    let filename;
    match _args.len().partial_cmp(&1).unwrap() {
        Ordering::Less => return Err("not enough args".to_string()),
        Ordering::Greater => {
            if _args.len() == 2 {
                path = PathBuf::from(_args.remove(0));
                filename = _args[0].clone();
            } else {
                return Err("too namy args".to_string());
            }
        }
        Ordering::Equal => {
            filename = _args[0].clone();
            path = PathBuf::from(".");
        }
    };
    Ok((filename, path))
}
*/
pub fn exec(filename : &str, path : &str) -> Result<(), String> {
    let path = PathBuf::from(path);
//    let (filename, path) = parse_args(_args)?;
    for file in search_in(&filename, path) {
        match file {
            Ok(file) => {
                println!("{:?}", file);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
    Ok(())
}
