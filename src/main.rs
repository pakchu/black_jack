use std::io;
fn main() {
    // intro
    println!("Welcom to Black Jack game!");
    println!("choose how many players you want to play with: ");
    
    // getting user input for # of platers
    let mut num_of_player_str = String::new();
    
    io::stdin().read_line(&mut num_of_player_str).unwrap();
    
    // poping '\n'
    num_of_player_str.pop();

    let num_of_player: i32 = num_of_player_str.parse().unwrap();

    println!("you chose {}", num_of_player);

    
}