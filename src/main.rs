mod blockchain;

#[macro_use]
extern crate serde_derive;

use std::io::{self, Write};
use std::process;
use crate::blockchain::Chain;

fn main() {
    let mut miner_address = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("input a miner addrress: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut miner_address).expect("invalid input");

    print!("Difficulty: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut difficulty).expect("invalid input");


    let difficulty = difficulty.trim().parse::<u32>().expect("we need an integer");
    miner_address = miner_address.trim().to_string();

    let mut chain = Chain::new(miner_address, difficulty);

    loop {
        println!("Menu");
        println!("1 - New Transaction");
        println!("2 - Mine block");
        println!("3 - Change Difficulty");
        println!("4 - Change Reward");
        println!("0 - Exit");

        io::stdout().flush().unwrap();
        choice.clear();
        io::stdin().read_line(&mut choice).expect("invalid input");
        println!();

        match choice.trim().parse().unwrap() {
            0 => exit(),
            1 => new_transaction(&mut chain),
            2 => mine_block(&mut chain),
            3 => change_difficulty(&mut chain),
            4 => change_reward(&mut chain),
            _ => invalid_option(),
        }
    }
}

fn exit() {
    println!("exiting!");
    process::exit(0);
}

fn new_transaction(chain: &mut Chain) {
    let mut sender = String::new();
    let mut receiver =  String::new();
    let mut amount = String::new();

    print!("enter sender address: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut sender).expect("invalid input");

    print!("enter receiver address: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut receiver).expect("invalid input");

    print!("enter amount: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut amount).expect("invalid input");

    let sender = sender.trim().to_string();
    let receiver =  receiver.trim().to_string();
    let amount = amount.trim().parse::<f32>().unwrap();

    let res = chain.new_transaction(sender, receiver, amount);

    match res {
        true => println!("transaction added"),
        false => println!("transaction failed"),
    }
}

fn mine_block(chain: &mut Chain) {
    println!("generating block");

    let res = chain.generate_new_block();

    match res {
        true => println!("block generated successfully"),
        false => println!("block generation failed"),
    }
}

fn change_difficulty(chain: &mut Chain) {
    let mut new_diff = String::new();
    print!("enter new difficulty: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut new_diff).expect("invalid input");

    let new_diff = new_diff.trim().parse::<u32>().unwrap();

    let res = chain.update_difficulty(new_diff);

    match res {
        true => println!("updated difficulty"),
        false => println!("failed updated difficulty"),
    }
}

fn change_reward(chain: &mut Chain) {
    let mut new_reward = String::new();
    print!("enter new reward: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut new_reward).expect("invalid input");

    let new_reward = new_reward.trim().parse::<f32>().unwrap();

    let res = chain.update_reward(new_reward);

    match res {
        true => println!("updated reward"),
        false => println!("failed updated reward"),
    }
}

fn invalid_option() {
    println!("\tinvalid option, please retry\t")
}

