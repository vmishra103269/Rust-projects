fn main() {
    if cfg!(windows) {
        println!("You are running a DOSbased system\n");
    } else if cfg!(unix) {
        println!("You are running a Unix based system\n");
    } else {
        println!("ERROR: Could not detect Operating System\n");
    }
}
