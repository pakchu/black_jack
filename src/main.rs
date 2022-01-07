use std::io::{self};
use rand::prelude::*;

#[derive(Debug, Clone)]
struct Trump<'a>{
    number: u8,
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

fn calculate(cards: &Vec<Trump>) -> u8 {
    let mut result: u8 = 0;
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

fn process_game<'game>(origin_deck: &mut Vec<Trump>, bet: i32, budget: &mut i32) -> i32 {
    let mut my_cards: Vec<Trump> = vec![];
    let mut dealer_cards: Vec<Trump> = vec![];

    // init trump deck (shuffled)
    let mut deck = init_trump_deck();
    *origin_deck = deck.clone();

    println!("--initial draw--");
    println!("---------------------------------");
    println!("you are getting card");
    my_cards.push(deck.pop().unwrap());
    println!("you got {:?}\n", my_cards[0]);
    
    println!("*********************************");
    println!("dealer getting Hidden card\nHIDDEN\n");
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
            println!("it is a push!");
            return 0;
        }
        else {
            println!("you got black jack!\nyou won!");
            return bet * 3 / 2;
        }
    }
    println!("your cards: {:?}\nsum: {}\n", my_cards, user_result);
    
    // ask for insurance
    if dealer_cards[1].number == 1 { 
        println!("insurance?");
    }
    
    let insurance: Option<bool> = if dealer_cards[1].number == 1 { 
        Some(user_choice())
    } 
    else { 
        None 
    };
    
    if insurance == Some(true) {
        println!("How much money will you insurance?\ninsurance must be smaller than 1/2 of bet\nyour bet: {}", bet);
        let mut insured: i32;
        loop {
            // getting user input for budget
            let mut insurance_str = String::new();
            
            io::stdin().read_line(&mut insurance_str).unwrap();
            
            // poping '\n'
            insurance_str.pop();

            insured = insurance_str.parse::<i32>().unwrap();

            if insured < 0 || 2 * insured > bet {
                println!("invalid insurance!");
                continue;
            }
            else {
                println!("you insured ${}", insured);
                match dealer_cards[0].number { 
                    10 | 11 | 12 | 13 => {
                        println!("dealer cards: {:?}\nsum: {}", dealer_cards, dealer_result);
                        println!("dealer's black jack!");
                        println!("but you insured");
                        return - bet + insured
                    },
                    _ => {
                        println!("dealer's pair is not a black jack\nyou lost your insuranced money");
                        *budget -= insured;
                    },
                }
                break;
            }
        }
    }

    // check for double down condition and ask
    if budget >= &mut (2 * bet) {
        println!("Double Down?");
    }

    let double_down:Option<bool> = if budget >= &mut (2 * bet) { Some(user_choice()) } else { None };

    if double_down == Some(true) {
        println!("---------------------------------");
        println!("you are getting card");
        my_cards.push(deck.pop().unwrap());
        let len = my_cards.len();
        println!("you got {:?}\n", my_cards[len-1]);

        user_result = calculate(&my_cards);

        if user_result > 21 {
            println!("dealer cards: {:?}\nsum: {}", dealer_cards, dealer_result);
            println!("your cards: {:?}\nsum: {}\n", my_cards, user_result);
            println!("you've bursted!");
            return -2 * bet;
        }

        if dealer_result == 21 {
            println!("dealer cards: {:?}\nsum: {}", dealer_cards, dealer_result);
            println!("your cards: {:?}\nsum: {}\n", my_cards, user_result);
            println!("dealer's black jack!");
            return -2 * bet * 2;
        }

        println!("&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&\nnow, dealer is drawing cards\n");
        while calculate(&dealer_cards) < 17 {
            println!("---------------------------------");
            println!("dealer getting card");
            dealer_cards.push(deck.pop().unwrap());
            let len = dealer_cards.len();
            println!("dealer's card {:?}\n", dealer_cards[len - 1]);
        }

        dealer_result = calculate(&dealer_cards);

        if dealer_result > 21 {
            println!("dealer's cards: {:?}\nsum: {}\n", dealer_cards, dealer_result);
            println!("dealer has bursted!\nyou won!");
            return 2 * bet;
        }

        println!("dealer's cards: {:?}\nsum: {}\n", dealer_cards, dealer_result);
        println!("your cards: {:?}\nsum: {}\n", my_cards, user_result);

        if user_result > dealer_result {
            println!("you won!");
            return 2 * bet;
        }
        else if user_result < dealer_result {
            println!("you lost!");
            return -2 * bet;
        }
        else {
            println!("its a push!");
            return 0;
        }
    }

    else {
        let mut choice = true;

        while calculate(&my_cards) < 21 && choice {
            println!("\nyour cards: {:?}\nsum: {}\n", my_cards, calculate(&my_cards));
            println!("hit?");
            choice = user_choice();
            if choice{
                println!("---------------------------------");
                println!("you are getting card");
                my_cards.push(deck.pop().unwrap());
                let len = my_cards.len();
                println!("you got {:?}\n", my_cards[len-1]);
            }
            else {
                println!("\nyou stayed\n");
            }
        }
        
        user_result = calculate(&my_cards);
        
        if user_result > 21 {
            println!("your cards: {:?}\nsum: {}\n", my_cards, user_result);
            println!("you've bursted!");
            return -bet;
        }

        if dealer_result == 21 {
            println!("its dealer's black jack!");
        }

        else if dealer_result < 17 {
            println!("&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&\nnow, dealer is drawing cards\n");
            while calculate(&dealer_cards) < 17 {
                println!("---------------------------------");
                println!("dealer getting card");
                dealer_cards.push(deck.pop().unwrap());
                let len = dealer_cards.len();
                println!("dealer's card {:?}\n", dealer_cards[len - 1]);
            }
        }

        dealer_result = calculate(&dealer_cards);

        if dealer_result > 21 {
            println!("dealer's cards: {:?}\nsum: {}\n", dealer_cards, dealer_result);
            println!("dealer has bursted!\nyou won!");
            return bet;
        }

        println!("dealer's cards: {:?}\nsum: {}\n", dealer_cards, dealer_result);
        println!("your cards: {:?}\nsum: {}\n", my_cards, user_result);

        if user_result > dealer_result {
            println!("you won!");
            return bet;
        }
        else if user_result < dealer_result {
            println!("you lost!");
            return -bet;
        }
        else {
            println!("its a push!");
            return 0;
        }
    }
}

fn main() {
    let mut origin_deck: Vec<Trump> = Vec::new();
    
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

    while bet > 0 && choice {
        println!("\nHow much will you bet?");
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
            println!("\ninvalid bet!");
            continue;
        }
        else {
            println!("your bet is larger than your budget: ${}. \nplease enter valid bet.", budget);
            continue;
        }
        let difference = process_game(&mut origin_deck, bet, &mut budget);
        budget += difference;
        println!("\nyour initial budget: ${}", budget_str.parse::<i32>().unwrap());
        println!("your last budget: ${}", budget - difference);
        println!("your current budget: ${}\n", budget);
        // println!("want to open deck?");
        // let print = user_choice();
        // if print {
        //     println!("deck was: {:?}\n", origin_deck);
        // }
        if budget == 0 {
            println!("you have no money. get out!");
            break;
        }
        else {
            println!("play one more time?");
            choice = user_choice();
        }
    }
    
}
