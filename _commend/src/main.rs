use std::process::Command;

fn main() {
    let ls = Command::new("ls").output().expect("Failed to execute ls");

    let output = String::from_utf8_lossy(&ls.stdout);
    println!("{output}");

    if output.contains("target") {
        let rm = Command::new("rm")
            .arg("-r")
            .arg("target")
            .status()
            .expect("Couldn't delete target");

        if rm.success() {
            println!("Successfully deleted lol");
        } else {
            println!("Failed to delete target");
        }
    } else {
        panic!("AAAAAAAA IM PANICKING CUZ THERES NO TARGET!!!");
    }
}
