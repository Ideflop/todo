use crate::{
    config::check_file_exist,
};

pub mod config;

use std::{
    env::args,   
    process::exit,
};
use crossterm_cursor::Result;

fn get_argument() -> String {
    let args = args().skip(1).collect::<Vec<_>>();
    exit(1);

    // display everything
    if args.is_empty() {
    }

    match args.get(0).unwrap().as_str() {
        //"-a" => create_alias(), // in config.rs
        _ => ()
    }

    let mut args = args.iter().peekable();
    let mut args_str = "".to_owned();
    while let Some(arg) = args.next() {
        args_str.push_str(&format!("+{}",arg));
    }
    args_str.remove(0);

    match args_str.as_str().trim() {
        //"-a" => create_alias(), // in config.rs
        "-u" => {
            let mut url_adrress  = args_str.clone();
            url_adrress.remove(0);
            }
        _ => ()
    }
    args_str

}

fn main() -> Result<()> {

    let config_path = check_file_exist(); // in config.rs
    
    let args = get_argument();

    // continuosly loop and wait for user input

    Ok(())
}
