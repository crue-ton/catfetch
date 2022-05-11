use std::env;
use sys_info;
use colored::*;
use std::process::{Command, Stdio};

fn main() {

// os {
    let os = match sys_info::linux_os_release() {
        Ok(info) => info.id.unwrap_or_default(),
        Err(_) => String::default(),
    };
// }
    

//shell {
    let shell = env::var("SHELL")
        .expect("Shell variable not found..");
    let shell = shell.split("/").last().unwrap();
//}

//wm {
    let output = Command::new("wmctrl")
        .arg("-m")
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let wm: Vec<&str> =  stdout.split_whitespace().collect();
    let wm = wm[1];
// }

// colors {
    let colors = "━━";
//}

    println!("
        /\\_/\\  {} {}
  _____/ 、  \\ {} {}
 /~____  =ø= / {} {}
(______)__m_m) {}{}{}{}{}{}", "os".red(), os, "sh".green(), shell, "wm".yellow(), wm, colors.red(), colors.green(), colors.yellow(), colors.blue(), colors.purple(), colors.cyan())

}
