use std::fs;
use std::cmp::Ordering;

pub fn exec(mut _args : Vec<String>) -> Result<(), String>{
    match _args.len().partial_cmp(&2).unwrap(){
        Ordering::Less => return Err("not enough args".to_string()),
        Ordering::Greater => return Err("too many args".to_string()),
        Ordering::Equal => (), 
    }
    let filename = _args.remove(1);
    let substring = _args.remove(0);

    let file_content = match fs::read_to_string(filename){
        Err(_) => return Err("Can't read the file".to_string()),
        Ok(content) => content,
    };
    for result_line in 
        file_content.lines()
            .filter(|line| line.contains(&substring)){
                println!("{}", result_line);
            }
    Ok(())
}
