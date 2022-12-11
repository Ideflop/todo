use std::{
    env,
    path::Path,
    thread::sleep,
    time::Duration,
    fs::{self,
        File,
    },
};


// check if file exist
pub fn check_file_exist() -> String{
    
    // check if config dir exist
    let config_home_result = env::var("XDG_CONFIG_HOME")
        .or_else(|_| env::var("HOME").map(|home|format!("{}/.config", home))).unwrap();
    let _config_exist = match Path::new(&config_home_result).exists() {
       true  => (),
       false => panic!("no config dir or no home variable set with XDG")
    };

    // check if todo dir exist inside the config file
    let todo_config_dir = env::var("XDG_CONFIG_HOME")
        .or_else(|_| env::var("HOME").map(|home|format!("{}/.config/todo", home))).unwrap();

    let todo_config_exist = Path::new(&todo_config_dir).exists();
    if !todo_config_exist {
        _ = create_config_todo_file(&todo_config_dir)
    }

    todo_config_dir.to_owned()+"/todo"
}

fn create_config_todo_file(path_to_todo_dir: &String) -> std::io::Result<()> {

    println!("no file or todo dir found.");
    println!("If the todo file should exist stop the programm and check if it's in {}", &path_to_todo_dir);
    println!("else just wait 3 seconds and the file will be created");
    sleep(Duration::from_secs(3));

    // create todo dir inside config dir
    fs::create_dir(&path_to_todo_dir)?;

    // create todo file inside todo dir
    let todo_file = path_to_todo_dir.to_owned()+"/todo";
    File::create(todo_file)?;

    println!("todo file created");

    Ok(())
}

