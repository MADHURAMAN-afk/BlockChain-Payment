#[macro_use]
extern crate serde_derive;

use std::io::{self, Write};
use blockchain::Chain;

mod blockchain;

fn main() {
    let mut miner_add = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("Input a miner address: ");
    io::stdout().flush().expect("Could not write to stdout");
    io::stdin().read_line(&mut miner_add).expect("Failed to read line");

    print!("Difficulty: ");
    io::stdout().flush().expect("Could not write to stdout");
    io::stdin().read_line(&mut difficulty).expect("Failed to read line");

    let diff = difficulty.trim().parse::<u32>().expect("we need an integer");
    println!("generating genesis block! ");
    let mut chain = Chain::new(miner_add.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine Block");
        println!("3) Change difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        println!("Enter your choice");

        io::stdout().flush().expect("Could not write to stdout");
        choice.clear();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("Exiting....");
                break;
            },
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                println!("Enter the sender address: ");
                io::stdout().flush().expect("Could not write to stdout");
                io::stdin().read_line(&mut sender).expect("Failed to read line");

                println!("Enter the receiver address: ");
                io::stdout().flush().expect("Could not write to stdout");
                io::stdin().read_line(&mut receiver).expect("Failed to read line");

                println!("Enter the amount: ");
                io::stdout().flush().expect("Could not write to stdout");
                io::stdin().read_line(&mut amount).expect("Failed to read line");

                let res = chain.new_transaction(sender.trim().to_string(), receiver.trim().to_string(), amount.trim().parse().unwrap());

                match res {
                    true => println!("Transaction was added"),
                    false => println!("Transaction failed"),
                }
            },
            2 => {
                println!("Generating a new block ");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block has been generated and added successfully"),
                    false => println!("Block generation failed"),
                }
            },
            3 => {
                let mut new_diff = String::new();
                println!("Please enter the difficulty: ");
                io::stdout().flush().expect("Could not write to stdout");
                io::stdin().read_line(&mut new_diff).expect("Failed to read line");
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Difficulty updated"),
                    false => println!("Something went wrong when updating the difficulty"),
                }
            },
            4 => {
                let mut new_reward = String::new();
                println!("Please enter the reward: ");
                io::stdout().flush().expect("Could not write to stdout");
                io::stdin().read_line(&mut new_reward).expect("Failed to read line");
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Reward updated"),
                    false => println!("Something went wrong when updating the reward"),
                }
            },
            _ => {
                println!("Invalid Option \n Please Enter Again: ");
            },
        }
    }
}
