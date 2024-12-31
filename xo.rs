use std::io;


static mut player1: String = String::new();
static mut player2: String = String::new();


fn intro() {
    println!("first player, enter your name :");
    io::stdin().read_line(&mut player1).expect("Failed to name player1");
    println!("second player, enter your name :");
    io::stdin().read_line(&mut player2).expect("Failed to name player2");


}



fn main() {

}