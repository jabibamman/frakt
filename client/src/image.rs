use std::{process::Command};

use shared::utils::filesystem::dir_exists;

pub fn open_image(path: &str) -> () {
    if !dir_exists(path) { 
        return;
     }
    
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/c", "start", path])
            .spawn()
            .unwrap();
    } else if cfg!(target_os = "linux") {
        Command::new("xdg-open").arg(path).spawn().unwrap();
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg(path).spawn().unwrap();
    } else {
        println!("OS not supported");
    }
}
