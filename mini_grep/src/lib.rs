use std::env;

pub enum Commands{
    Cat,
    Grep,
    Find,
}

impl Commands{
    pub fn parse_command(string : &str) -> Result<Self,&'static str>{      
        match string{
            "cat" => Ok(Self::Cat),
            "grep" => Ok(Self::Grep),
            "find" => Ok(Self::Find),
            _ => Err("Can't recognize command"),
        }
    }
}

pub struct Config{
    pub _bin : String,
    pub _command : Commands,
    pub _args : Vec<String>,
}


impl Config{
    pub fn new(mut args : env::Args) -> Result<Self, &'static str>{
        let bin = match args.next(){
            Some(i) => i,
            None => return Err("How??"),
        };
        let mut args : Vec<String> = args.collect();
        let command = Commands::parse_command(&args.remove(0))?;
        
        Ok(
            Config {
            _bin : bin,
            _command : command,
            _args : args,
        }
        )
    }
}

mod cat;
mod find;
mod grep;
pub fn run(config : Config) -> Result<(), String>{
    match config._command {
        Commands::Cat => cat::exec(config._args),
        Commands::Find => find::exec(config._args),
        Commands::Grep => grep::exec(config._args),
    }
}

#[cfg(test)]
mod tests;
