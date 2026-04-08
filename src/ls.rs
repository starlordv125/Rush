// Lists contents in directories

use std::fs;
use colored::Colorize;
use std::path::Path;

pub fn list() {
    let list = fs::read_dir("./").unwrap();
        for item in list {
            let pretty_item = item.unwrap().path().display().to_string().replace("./", "");
            if Path::new(&pretty_item).is_file() {
                println!("{}", pretty_item.bright_green());
            } else if Path::new(&pretty_item).is_dir() {
                println!("{}", pretty_item.blue());
            } else {
                println!("{}", pretty_item)
            }
        }
}
