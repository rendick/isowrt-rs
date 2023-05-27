use std::io;
use std::process::Command;

fn main() {
    // whoami
    let home_output = Command::new("whoami")
        .output()
        .expect("Failed to execute command");

    let home_stgout = String::from_utf8_lossy(&home_output.stdout);

    // ls
    let ls_output = Command::new("ls")
        .arg(format!("/home/{}/", home_stgout.trim()))
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
    io::stdin().read_line(&mut iso_folder).expect("Failed to read input");

    iso_folder = iso_folder.trim().to_string();

    println!("\nYou selected: {}\n", iso_folder);

    println!("All ISO files in /home/{}/{}:", home_stgout.trim(), iso_folder);

    // find
    let find_output = Command::new("find")
        .arg(format!("/home/{}/{}", home_stgout.trim(), iso_folder))
        .arg("-type")
        .arg("f")
        .arg("-name")
        .arg("*.iso")
        .output()
        .expect("Failed to execute command");

    let find_stdout = String::from_utf8_lossy(&find_output.stdout);
    println!("{}", find_stdout);

    let mut iso_name = String::new();
    println!("Enter the ISO path: ");
    io::stdin()
        .read_line(&mut iso_name)
        .expect("Failed to read input");

    println!("\nYou selected this ISO: {}", iso_name);

    // lsblk
    let lsblk_output = Command::new("lsblk")
        .output()
        .expect("Failed to execute command");

    let lsblk_stdout = String::from_utf8_lossy(&lsblk_output.stdout);

    println!("{}", lsblk_stdout);

    let mut lsblk_name = String::new();
    println!("Enter your flash drive name: ");
    io::stdin()
        .read_line(&mut lsblk_name)
        .expect("Failed to read input");

    println!(
        "Your flash drive name: {}", 
        lsblk_name.trim()
    );

    // main command
    let dd_output = Command::new("sudo")
        .arg("dd")
        .arg(format!("if={}", iso_name.trim() ))
        .arg(format!("of={}", lsblk_name.trim()))
        .arg("bs=4M")
        .arg("status=progress")
        .arg("&&")
        .arg("sync")
        .output()
        .expect("Failed to execute dd command");

    println!("{:?}", dd_output);

    let dd_stdout = String::from_utf8_lossy(&dd_output.stdout);
    println!("{}", dd_stdout);

}
