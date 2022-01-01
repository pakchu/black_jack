use std::io::{self};
use rand::prelude::*;

#[derive(Debug)]
struct Trump<'a>{
    number: u32,
    shapes: &'a str,
}

fn user_choice() -> bool {   
    loop{
        println!("y/n ?");
        
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).unwrap();

        if answer == "y\n".to_string() {
            return true;
        }
        else if answer == "n\n".to_string() {
            return false;
        }
        else {
            println!("*** invalid answer! ***");
        }
    }
}

fn init_trump_deck() -> Vec<Trump<'static>> {
    let mut deck: Vec<Trump> = Vec::new();
    for num in 1..=13{
        for shape in vec!["spade", "clover", "heart", "diamond"] {
            deck.push(Trump{
                number : num,
                shapes : shape,
            });
        }
    }
    let mut rng = rand::thread_rng();

    deck.shuffle(&mut rng);

    return deck;
}

fn calculate(cards: &Vec<Trump>) -> u32 {
    let mut result: u32 = 0;
    let mut num_of_aces: u32 = 0;
    for card in cards{
        if card.number == 1 {
            num_of_aces += 1;
            result += 11;
        }
        else if card.number > 10 {
            result += 10;
        }
        else {
            result += card.number;
        }
    }

    while result > 21 && num_of_aces != 0 {
        result -= 10;
        num_of_aces -= 1;
    }

    result
}

fn process_game<'game>(bet: i32 ) -> i32 {
    let mut my_cards: Vec<Trump> = vec![];
    let mut dealer_cards: Vec<Trump> = vec![];

    // init trump deck (shuffled)
    let mut deck = init_trump_deck();

    println!("--initial draw--");
    println!("---------------------------------");
    println!("you are getting card");
    my_cards.push(deck.pop().unwrap());
    println!("you got {:?}\n", my_cards[0]);
    
    println!("*********************************");
    println!("dealer getting card\n");
    dealer_cards.push(deck.pop().unwrap());

    println!("---------------------------------");
    println!("you are getting card");
    my_cards.push(deck.pop().unwrap());
    println!("you got {:?}\n", my_cards[1]);

    println!("*********************************");
    println!("dealer getting card");
    dealer_cards.push(deck.pop().unwrap());
    println!("dealer's open card {:?}\n", dealer_cards[1]);

    let mut user_result = calculate(&my_cards);
    let mut dealer_result = calculate(&dealer_cards);

    if user_result == 21 {
        println!("dealer's cards: {:?}", dealer_cards);
        println!("your cards: {:?}", my_cards);
        if dealer_result == 21 {
            println!("it is a draw!");
            return 0;
        }
        else {
            println!("you got black jack!\nyou won!");
            return bet * 3 / 2;
        }
    }
    else if dealer_result == 21 {
        println!("dealer's cards: {:?}", dealer_cards);
        println!("your cards: {:?}", my_cards);
        println!("dealer's cards are black jack!\nyou lost");
        return -bet;
    }

    let mut choice = true;

    while calculate(&my_cards) < 21 && choice {
        println!("your cards: {:?}\nsum: {}", my_cards, calculate(&my_cards));
        println!("hit?");
        choice = user_choice();
        if choice{
            println!("---------------------------------");
            println!("you are getting card");
            my_cards.push(deck.pop().unwrap());
            let len = my_cards.len();
            println!("you got {:?}\n", my_cards[len-1]);
        }
    }
    user_result = calculate(&my_cards);

    if user_result > 21 {
        println!("your cards: {:?}\nsum: {}", my_cards, user_result);
        println!("you've bursted!");
        return -bet;
    }

    println!("&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&\nnow, dealer is drawing cards\n");
    while calculate(&dealer_cards) < 17 {
        println!("---------------------------------");
        println!("dealer getting card");
        dealer_cards.push(deck.pop().unwrap());
        let len = dealer_cards.len();
        println!("dealer's open card {:?}\n", dealer_cards[len - 1]);
    }

    dealer_result = calculate(&dealer_cards);

    if dealer_result > 21 {
        println!("dealer's cards: {:?}\nsum: {}", dealer_cards, dealer_result);
        println!("dealer has bursted!\nyou won!");
        return bet;
    }

    println!("dealer's cards: {:?}\nsum: {}", dealer_cards, dealer_result);
    println!("your cards: {:?}\nsum: {}", my_cards, user_result);

    if user_result > dealer_result {
        println!("you won!");
        return bet
    }
    else if user_result < dealer_result {
        println!("you lost!");
        return -bet;
    }
    else {
        println!("its a draw!");
        return 0;
    }

}

fn main() {
    // intro
    println!("Welcom to Black Jack game!");
    println!("How much money you have: ");
    
    // getting user input for budget
    let mut budget_str = String::new();
    
    io::stdin().read_line(&mut budget_str).unwrap();
    
    // poping '\n'
    budget_str.pop();

    let mut budget: i32 = budget_str.parse().unwrap();

    if budget <= 0 {
        println!("get out of here!");
        panic!();
    }

    println!("you have ${}", budget);

    let mut bet: i32 = 1;

    let mut choice = true;

    while bet > 0 && choice{
        println!("how much will you bet?");
        // getting user input for bet amount
        let mut bet_str = String::new();
        
        io::stdin().read_line(&mut bet_str).unwrap();
        
        // poping '\n'
        bet_str.pop();

        bet = bet_str.parse().unwrap();
        if bet <= budget && bet > 0 {
            println!("your current bet: ${}", bet);
        }
        else if !(bet > 0) {
            println!("invalid bet!");
            continue;
        }
        else {
            println!("your bet is larger than your budget. \nplease enter valid bet.");
            continue;
        }
        budget += process_game(bet);
        println!("\nyour initial budget: ${}", budget_str.parse::<i32>().unwrap());
        println!("your current budget: ${}\n", budget);
        println!("play one more time?");
        choice = user_choice();
    }
}
