use std::io;
use std::process::Command;

fn main() {
    let home = String::from("whoami");

    let home_output = Command::new(home)
        .output()
        .expect("Failed to execute command");

    let home_stgout = String::from_utf8_lossy(&home_output.stdout);

    println!("stdout: {}", home_stgout);

    let ls = String::from("ls");
    let ls_args = format!("/home/{}/Project/", home_stgout.trim());

    let ls_output = Command::new(ls)
        .arg(&ls_args)
        .output()
        .expect("Failed to execute command");

    let ls_stdout = String::from_utf8_lossy(&ls_output.stdout);

    println!("stdout: {}", ls_stdout);

    let mut iso_folder = String::new();

    println!(
        "Enter the folder where the iso is located (The folder must be in {}):",
        home_stgout.trim()
    );
    io::stdin().read_line(&mut iso_folder).expect("Hello");

    println!("hello: {}", iso_folder)
    
}
