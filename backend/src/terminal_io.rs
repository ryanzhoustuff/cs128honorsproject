use crate::cards::Card;
use crate::io_interface::{GameAction, GameIO};
use std::io::{self, Write};

pub struct TerminalIO;

impl TerminalIO {
    pub fn new() -> Self {
        TerminalIO
    }

    fn get_amount(&self, player_name: &str) -> u32 {
        loop {
            print!("{} enter amount: ", player_name);
            io::stdout().flush().unwrap();
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            if let Ok(num) = line.trim().parse::<u32>() {
                return num;
            } else {
                println!("Invalid number, try again.");
            }
        }
    }
}

impl GameIO for TerminalIO {
    fn get_player_action(&mut self, player_name: &str, to_call: u32, can_check: bool) -> GameAction {
        loop {
            if can_check && to_call == 0 {
                println!("{}'s turn. Options: check, bet, fold", player_name);
            } else if to_call == 0 {
                println!("{}'s turn. Options: bet, fold", player_name);
            } else {
                println!("{}'s turn. To call: {} chips. Options: fold, call, raise", player_name, to_call);
            }

            print!("Enter action: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let action = input.trim().to_lowercase();

            if action == "fold" {
                return GameAction::Fold;
            }
            if can_check && to_call == 0 {
                if action == "check" {
                    return GameAction::Check;
                } else if action == "bet" {
                    let amount = self.get_amount(player_name);
                    if amount > 0 {
                        return GameAction::Bet(amount);
                    } else {
                        println!("Bet must be > 0");
                    }
                }
            } else if to_call == 0 {
                if action == "bet" {
                    let amount = self.get_amount(player_name);
                    if amount > 0 {
                        return GameAction::Bet(amount);
                    } else {
                        println!("Bet must be > 0");
                    }
                }
            } else {
                if action == "call" {
                    return GameAction::Call;
                } else if action == "raise" {
                    let amount = self.get_amount(player_name);
                    if amount > 0 {
                        return GameAction::Raise(amount);
                    } else {
                        println!("Raise must be > 0");
                    }
                }
            }
            println!("Invalid action. Try again.");
        }
    }

    fn show_message(&mut self, message: &str) {
        println!("{}", message);
    }

    fn show_game_state(
        &mut self,
        pot: u32,
        community_cards: &[Card],
        players: &[(String, u32, u32, bool, Vec<Card>)],
        current_player: &str,
    ) {
        println!("\n--- Game State ---");
        println!("Pot: {}", pot);
        print!("Community Cards: ");
        for c in community_cards {
            print!("{} ", c);
        }
        println!();

        for (name, chips, cur_bet, folded, hole_cards) in players {
            println!("{}: {} chips | Current Bet: {} | Folded: {}", name, chips, cur_bet, folded);
            print!("Hole Cards: ");
            for card in hole_cards {
                print!("{} ", card);
            }
            println!();
        }

        println!("Current Player: {}", current_player);
        println!("------------------\n");
    }
}
