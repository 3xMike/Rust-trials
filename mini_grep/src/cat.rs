use std::fs;

pub fn exec(mut _args : Vec<String>) -> Result<(), String>{
    if _args.len() == 0 {
        return Err("not enough args".to_string());
    }

    for file in _args {
        let file_content = match fs::read_to_string(file.clone()){
            Err(_) => return Err(format!("Can't read the file {}", file)),
            Ok(content) => content,
        };
        println!("{}",file_content);
        //if recording not in stdout, change prev line
    }
    Ok(())
}
