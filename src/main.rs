use std::io;
use rand::prelude::*;

struct Trumps<'a>{
    number: u32,
    shapes: &'a str,
}

fn init_trump_deck() -> Vec<Trumps<'static>> {
    let mut deck: Vec<Trumps> = Vec::new();
    for num in 1..13{
        for shape in vec!["spade", "clover", "heart", "diamond"] {
            deck.push(Trumps{
                number : num,
                shapes : shape,
            });
        }
    }
    let mut rng = rand::thread_rng();

    deck.shuffle(&mut rng);

    return deck;
}

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
    
    // init trump deck (shuffled)
    init_trump_deck();

    
}