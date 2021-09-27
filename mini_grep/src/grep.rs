use std::fs;

pub fn exec<'a>(filename : &str, substring : &str) -> Result<(), String>{

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
