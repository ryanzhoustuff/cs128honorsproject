use crate::cards::Card;

pub trait GameIO {
    fn get_player_action(&mut self, player_name: &str, to_call: u32, can_check: bool) -> GameAction;
    fn show_message(&mut self, message: &str);
    fn show_game_state(
        &mut self,
        pot: u32,
        community_cards: &[Card],
        players: &[(String, u32, u32, bool, Vec<Card>)],
        current_player: &str,
    );
}

#[derive(Debug)]
pub enum GameAction {
    Fold,
    Check,
    Call,
    Bet(u32),
    Raise(u32),
}
