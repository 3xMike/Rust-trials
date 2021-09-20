use mini_grep::Config;

fn main() {
    let config = 
        match Config::new(std::env::args()){
            Ok(v) => v,
            Err(e) => panic!("{}",e),
        };
    if let Err(e) = mini_grep::run(config){
        println!{"running error: {}", e};
        std::process::exit(-1);
    }
}
