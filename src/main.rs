use std::io;
use std::process::Command;

fn main() {

    let home = String::from("whoami");

    let home_output = Command::new(home)
        .output()
        .expect("Failed to execute command");

    let home_stgout = String::from_utf8_lossy(&home_output.stdout);

    let ls = String::from("ls");
    let ls_args = format!("/home/{}/", home_stgout.trim());

    let ls_output = Command::new(ls)
        .arg(&ls_args)
        .output()
        .expect("Failed to execute command");

    let ls_stdout = String::from_utf8_lossy(&ls_output.stdout);

    println!("Folders in /home/{}", home_stgout.trim());
    println!("{}", ls_stdout);

    let mut iso_folder = String::new();

    println!(
        "Enter the folder where the iso is located (The folder must be in {}):",
        home_stgout.trim()
    );
    io::stdin().read_line(&mut iso_folder).expect("Hello");

    println!("You select: {}", iso_folder);

    let lsblk = String::from("lsblk");

    let lsblk_output = Command::new(lsblk)
        .output()
        .expect("failed");

    let lsblk_stdout = String::from_utf8_lossy(&lsblk_output.stdout);

    println!("{}", lsblk_stdout);

    let mut lsblk_name = String::new();
    println!("Enter your flash drive name: ");
    io::stdin().read_line(&mut lsblk_name).expect("failed to check your flash drive name");
    println!("Your flash drive name: {}", lsblk_name.trim());

}