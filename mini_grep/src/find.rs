use std::io;
use std::path::PathBuf;
mod recurses;

fn search_in<'a>(
    filename: &'a str,
    path: PathBuf,
    threads: usize,
) -> Box<dyn Iterator<Item = io::Result<PathBuf>> + 'a> {
    if threads == 1 {
        Box::new(recurses::recurse_dir(path).filter(move |x| {
            match x {
                Err(_) => true,
                Ok(path) => {
                    path.file_name()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or_default()
                        == filename
                }
            }
        }))
    } else {
        Box::new(recurses::par_recurse(path, threads).into_iter()
        .filter(move |x| {
            match x {
                Err(_) => true,
                Ok(path) => {
                    path.file_name()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or_default()
                        == filename
                }
            }
        }))
    }
}

pub fn exec(filename: &str, path: &str, threads: usize) -> Result<(), String> {
    let path = PathBuf::from(path);

    for file in search_in(&filename, path, threads) {
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
