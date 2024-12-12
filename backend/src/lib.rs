pub mod cards;
pub mod hand_evaluator;
pub mod io_interface;
pub mod terminal_io;
pub mod game;

pub fn run_heads_up_game(starting_bb: u32) {
    let mut game = game::Game::new(starting_bb);
    let mut io = terminal_io::TerminalIO::new();
    game.play_hand(&mut io);
}
