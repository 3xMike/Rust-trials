use std::fs;

pub fn exec<'a>(args : impl Iterator<Item = &'a str>) -> Result<(), String>{
    for file in args {
        let file_content = match fs::read_to_string(file.clone()){
            Err(_) => return Err(format!("Can't read the file {}", file)),
            Ok(content) => content,
        };
        println!("{}",file_content);
        //if recording not in stdout, change prev line
    }
    Ok(())
}
