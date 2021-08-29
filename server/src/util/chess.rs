use chess::{Board, MoveGen};
use std::collections::HashMap;

pub fn get_dests(board: &Board) -> HashMap<String, String> {
    let mut movegen = MoveGen::new_legal(&board);
    let mut dests: HashMap<String, String> = HashMap::new();

    for chess_move in movegen {
        match dests.get_mut(&chess_move.get_source().to_string()) {
            Some(dest) => {
                dest.push_str(&chess_move.get_dest().to_string());
            }
            None => {
                dests.insert(
                    chess_move.get_source().to_string(),
                    chess_move.get_dest().to_string(),
                );
            }
        }
    }

    dests
}
