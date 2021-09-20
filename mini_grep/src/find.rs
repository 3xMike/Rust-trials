use std::fs;
use std::path::{Path,PathBuf};
use std::error::Error;
use std::collections::VecDeque;
use std::cmp::Ordering;

fn search_in(filename : &String, path : &Path) ->
                        Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut queue = VecDeque::new();
    let mut result = Vec::new();

    queue.push_back( fs::read_dir(path)? );
    while !queue.is_empty() {
        for entry in queue.pop_front(){// entry - ReadDir
            for dir_entry in entry{//dir_entry - Result<DirEntry>
                let dir_entry = dir_entry?;
                if dir_entry
                .file_name().into_string()
                .expect("Can't read OsString") == *filename{
                    result.push(dir_entry.path());
                }
                if dir_entry
                .file_type().expect("Can't determine file_type")
                .is_dir(){

                    queue.push_back(fs::read_dir(dir_entry.path().as_path())?);
                }
            }
        }
    }
    Ok(result)
}

fn parse_args(mut _args : Vec<String>) -> Result<(String,String), String>{
    let path : String;
    let filename : String;
    match _args.len().partial_cmp(&1).unwrap(){
            Ordering::Less => return Err("not enough args".to_string()),
            Ordering::Greater => if _args.len() == 2{
                path = _args.remove(0);
                filename = _args[0].clone();
            } else {
                return Err("too namy args".to_string());
            }
            Ordering::Equal => {
                filename = _args[0].clone();
            path = ".".to_string();
            }
        };
    Ok( (filename, path) )
}

pub fn exec(mut _args : Vec<String>) -> Result<(), String>{
    let (filename, path) = parse_args(_args)?;
    let result = search_in(&filename, Path::new(&path)).expect("Bruh");
    if result.is_empty(){
        println!("No such file in directory");
        return Ok(());
    } else {
        for path in result{
          println!("{:?}", path);  
        }
    }
    Ok(())
}
