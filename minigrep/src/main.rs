use std::env;
use std::fs;
use minigrep::Config


fn main(){
    let args:Vec<String>=env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Pronlem parsing argument:{err}");
        process::exit();
    })

    if let Err(e) = minigrep::run(config){
        println("Application error: {e}");
        process::exit(1);
    }
}

