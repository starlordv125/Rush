use text_io::read;
use std::{env, path::Path};

pub fn change_dir() {
    print!("Directory to change to: ");
    let input_dir: String = read!();
    let root = Path::new(&input_dir);
    assert!(env::set_current_dir(root).is_ok());
}