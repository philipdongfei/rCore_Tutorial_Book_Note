
use std::{env, fs};

fn main() {
    //let current_dir = env::current_dir().unwrap();
    match fs::read_dir(".") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            if let Ok(path) = path {
                if let Ok(file_type) = path.file_type() {
                    if file_type.is_file(){
                        println!("{:?}", path.path());
                    }
                }
            }
        }
    }
}
