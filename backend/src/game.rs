use crate::cards::{Card, Deck};
use crate::hand_evaluator::evaluate_hand;
use crate::io_interface::{GameIO, GameAction};
use std::cmp::min;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub chips: u32,
    pub hand: Vec<Card>,
    pub folded: bool,
    pub current_bet: u32,
}

impl Player {
    fn new(name: &str, chips: u32) -> Self {
        Player {
            name: name.to_string(),
            chips,
            hand: Vec::new(),
            folded: false,
            current_bet: 0,
        }
    }

    fn bet(&mut self, amount: u32) -> u32 {
        let actual = min(self.chips, amount);
        self.chips -= actual;
        self.current_bet += actual;
        actual
    }

    fn reset_for_new_hand(&mut self) {
        self.hand.clear();
        self.folded = false;
        self.current_bet = 0;
    }
}

pub struct Game {
    pub players: Vec<Player>,
    pub deck: Deck,
    pub community_cards: Vec<Card>,
    pub pot: u32,
    pub big_blind: u32,
    pub small_blind: u32,
    pub dealer_button: usize,
    pub current_player: usize,
    pub highest_bet: u32,
}

impl Game {
    pub fn new(starting_bb: u32) -> Self {
        let bb_amount = 2;
        let sb_amount = 1;
        let starting_chips = starting_bb * bb_amount;

        let players = vec![
            Player::new("Player 1", starting_chips),
            Player::new("Player 2", starting_chips),
        ];

        let mut deck = Deck::new();
        deck.shuffle();

        Game {
            players,
            deck,
            community_cards: Vec::new(),
            pot: 0,
            big_blind: bb_amount,
            small_blind: sb_amount,
            dealer_button: 0,
            current_player: 0,
            highest_bet: 0,
        }
    }

    pub fn play_hand(&mut self, io: &mut impl GameIO) {
        self.reset_hand_state();
        self.deal_hole_cards();
        self.post_blinds();

        // Preflop
        if !self.betting_round(io) {
            // fold preflop
            self.award_pot_to_last_standing(io);
            return;
        }
        println!("Preflop betting round ended normally, dealing flop next...");
        if self.active_players_count() < 2 {
            self.award_pot_to_last_standing(io);
            return;
        }
        println!("Proceeding to flop...");

        // Flop
        self.deal_community_cards(3);
        if !self.betting_round(io) {
            self.award_pot_to_last_standing(io);
            return;
        }
        if self.active_players_count() < 2 {
            self.award_pot_to_last_standing(io);
            return;
        }
        println!("Proceeding to turn...");

        // Turn
        self.deal_community_cards(1);
        if !self.betting_round(io) {
            self.award_pot_to_last_standing(io);
            return;
        }
        if self.active_players_count() < 2 {
            self.award_pot_to_last_standing(io);
            return;
        }
        println!("Proceeding to river...");

        // River
        self.deal_community_cards(1);
        if !self.betting_round(io) {
            self.award_pot_to_last_standing(io);
            return;
        }
        if self.active_players_count() < 2 {
            self.award_pot_to_last_standing(io);
            return;
        }

        // Showdown
        self.showdown(io);
    }

    fn reset_hand_state(&mut self) {
        self.community_cards.clear();
        self.pot = 0;
        self.highest_bet = 0;
        self.deck = Deck::new();
        self.deck.shuffle();
        for p in &mut self.players {
            p.reset_for_new_hand();
        }
        self.current_player = (self.dealer_button + 1) % 2;
    }

    fn deal_hole_cards(&mut self) {
        for _ in 0..2 {
            for p in &mut self.players {
                if let Some(card) = self.deck.deal() {
                    p.hand.push(card);
                }
            }
        }
    }

    fn deal_community_cards(&mut self, count: usize) {
        self.deck.deal(); // burn card
        for _ in 0..count {
            if let Some(card) = self.deck.deal() {
                self.community_cards.push(card);
            }
        }
        self.prepare_for_betting_round();
    }

    fn post_blinds(&mut self) {
        let sb_index = self.dealer_button;
        let bb_index = (self.dealer_button + 1) % 2;

        self.pot += self.players[sb_index].bet(self.small_blind);
        self.pot += self.players[bb_index].bet(self.big_blind);

        self.highest_bet = self.big_blind;
        self.current_player = sb_index; // small blind acts first preflop
    }

    fn prepare_for_betting_round(&mut self) {
        self.highest_bet = 0;
        for p in &mut self.players {
            p.current_bet = 0;
        }
        // After flop/turn/river, first to act is always the player who was SB preflop
        self.current_player = self.dealer_button % 2;
    }

    fn betting_round(&mut self, io: &mut impl GameIO) -> bool {
        let mut actions_this_round = 0;
    
        loop {
            if self.active_players_count() == 1 {
                // Fold detected
                return false;
            }
    
            // If all bets are equal and we've had at least as many actions as active players,
            // it means everyone got a chance to respond.
            if self.all_bets_equalized() && actions_this_round >= self.active_players_count() {
                return true;
            }
    
            let idx = self.current_player;
            if self.players[idx].folded {
                self.next_player();
                continue;
            }
    
            self.show_game_state(io);
            let to_call = self.highest_bet.saturating_sub(self.players[idx].current_bet);
            let can_check = to_call == 0;
    
            let action = io.get_player_action(&self.players[idx].name, to_call, can_check);
    
            let fold_happened = !self.handle_action(idx, &action, io);
            if fold_happened && self.active_players_count() == 1 {
                return false;
            }
    
            // **Remove the dereference operator here**
            match action {
                GameAction::Bet(_) | GameAction::Raise(_) => {
                    actions_this_round = 1; // reset since a new bet/raise happened
                }
                _ => {
                    actions_this_round += 1;
                }
            }
    
            self.next_player();
        }
    }    

    fn handle_action(&mut self, player_idx: usize, action: &GameAction, io: &mut impl GameIO) -> bool {
        match action {
            &GameAction::Fold => {
                self.players[player_idx].folded = true;
                io.show_message(&format!("{} folds.", self.players[player_idx].name));
                false
            }
            &GameAction::Check => {
                io.show_message(&format!("{} checks.", self.players[player_idx].name));
                true
            }
            &GameAction::Call => {
                let to_call = self.highest_bet - self.players[player_idx].current_bet;
                let actual_call = min(to_call, self.players[player_idx].chips);
                self.players[player_idx].chips -= actual_call;
                self.players[player_idx].current_bet += actual_call;
                self.pot += actual_call;
                io.show_message(&format!("{} calls {}.", self.players[player_idx].name, actual_call));
                true
            }
            &GameAction::Bet(amount) => {
                if amount > self.players[player_idx].chips {
                    let all_in = self.players[player_idx].chips;
                    self.players[player_idx].chips = 0;
                    self.players[player_idx].current_bet += all_in;
                    self.pot += all_in;
                    self.highest_bet = self.players[player_idx].current_bet;
                    io.show_message(&format!("{} goes all-in for {}.", self.players[player_idx].name, all_in));
                } else {
                    self.players[player_idx].chips -= amount;
                    self.players[player_idx].current_bet += amount;
                    self.pot += amount;
                    self.highest_bet = self.players[player_idx].current_bet;
                    io.show_message(&format!("{} bets {}.", self.players[player_idx].name, amount));
                }
                true
            }
            &GameAction::Raise(amount) => {
                let to_call = self.highest_bet.saturating_sub(self.players[player_idx].current_bet);
                let total_needed = to_call + amount;
                if total_needed > self.players[player_idx].chips {
                    let all_in = self.players[player_idx].chips;
                    self.players[player_idx].chips = 0;
                    self.players[player_idx].current_bet += all_in;
                    self.pot += all_in;
                    self.highest_bet = self.players[player_idx].current_bet;
                    io.show_message(&format!("{} goes all-in for {}.", self.players[player_idx].name, all_in));
                } else {
                    self.players[player_idx].chips -= total_needed;
                    self.players[player_idx].current_bet += total_needed;
                    self.pot += total_needed;
                    self.highest_bet = self.players[player_idx].current_bet;
                    io.show_message(&format!("{} raises to {}.", self.players[player_idx].name, self.players[player_idx].current_bet));
                }
                true
            }
        }
    }    

    fn all_bets_equalized(&self) -> bool {
        let active_bets: Vec<u32> = self.players.iter().filter(|p| !p.folded).map(|p| p.current_bet).collect();
        if active_bets.is_empty() {
            return true;
        }
        let first = active_bets[0];
        active_bets.iter().all(|&b| b == first)
    }

    fn active_players_count(&self) -> usize {
        self.players.iter().filter(|p| !p.folded).count()
    }

    fn next_player(&mut self) {
        self.current_player = (self.current_player + 1) % 2;
    }

    fn award_pot_to_last_standing(&mut self, io: &mut impl GameIO) {
        if let Some(winner) = self.players.iter_mut().find(|p| !p.folded) {
            io.show_message(&format!("{} wins {} chips (opponent folded)!", winner.name, self.pot));
            winner.chips += self.pot;
            self.pot = 0;
        }
    }

    fn showdown(&mut self, io: &mut impl GameIO) {
        let mut best_rank = None;
        let mut winner = None;

        let mut results = Vec::new();

        for (i, player) in self.players.iter().enumerate() {
            if player.folded {
                continue;
            }
            let mut full_hand = player.hand.clone();
            full_hand.extend_from_slice(&self.community_cards);
            let rank = evaluate_hand(&full_hand);
            results.push((i, rank));
        }

        for &(i, rank) in &results {
            if best_rank.is_none() || rank > best_rank.unwrap() {
                best_rank = Some(rank);
                winner = Some(i);
            }
        }

        for &(i, rank) in &results {
            io.show_message(&format!("{} has: {:?}", self.players[i].name, rank));
        }

        if let Some(wi) = winner {
            io.show_message(&format!("{} wins {} chips!", self.players[wi].name, self.pot));
            self.players[wi].chips += self.pot;
            self.pot = 0;
        }
    }

    fn show_game_state(&self, io: &mut impl GameIO) {
        let players_info: Vec<(String, u32, u32, bool, Vec<Card>)> = self.players.iter()
            .map(|p| (p.name.clone(), p.chips, p.current_bet, p.folded, p.hand.clone()))
            .collect();

        io.show_game_state(
            self.pot,
            &self.community_cards,
            &players_info,
            &self.players[self.current_player].name,
        );
    }
}
