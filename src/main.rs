#[macro_use]
extern crate serde_derive;

use std::io;
use std::io::stdin;
use std::process;
use std::io::Write;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    println!("Input minder address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_addr);
    println!("Input Difficulty: ");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("Please input an Integer");
    println!("Generating Genesis Block!");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine Block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        println!("Enter your choice: ");
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
        println!("");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("Exiting! ...");
                process::exit(0);
            },
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                println!("Enter Sender Address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);

                println!("Enter Receiver Address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut receiver);

                println!("Enter Sender Address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                let res = chain.new_transaction(sender.trim().to_string(), receiver.trim().to_string(), amount.trim().parse().unwrap());

                match res {
                    true => println!("Transaction Added!"),
                    false => println!("Transaction Failed!"),
                }
            },
            2 => {
                println!("Generating Block!");
                let res = chain.generate_new_block();
                
                match res {
                    true => println!("Block Generated Successfully!"),
                    false => println!("Block Generated Failed."),
                }
            }, 
            3 => {
                let mut new_diff = String::new();
                println!("Enter new Difficulty: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_diff);
                chain.update_difficulty(new_diff.trim().parse().unwrap());

                match res {
                    true => println!("Updated Difficulty!"),
                    false => println!("Failed to update Difficulty."),
                }
            },
            4 => {
                let mut new_reward = String::new();
                println!("Enter new Reward Amount: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);
                chain.update_reward(new_reward.trim().parse().unwrap());

                match res {
                    true => println!("Updated Reward!"),
                    false => println!("Failed to update Reward."),
                }
            },
            _ => print!("Invalid Option, please try again \t"),
        }
    }
}
