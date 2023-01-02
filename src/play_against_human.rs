use crate::{
    game::{steps_to_go_heuristic, State},
    util::time,
};
use std::fs;

#[allow(unused)]
pub fn play_against_human() {
    let initial_state = fs::read_to_string("./input.txt")
        .unwrap()
        .parse::<State>()
        .unwrap();
    // .with_turn(XULOS);

    // let initial_state = State::initial();

    println!("Initial state:",);
    println!("========");
    initial_state.viz(0);

    time(|| {
        let num_steps = 10;
        let (moves, _) =
            initial_state.minimax(num_steps, true, |state| steps_to_go_heuristic(&state));

        println!("Best move for me, looking {num_steps} steps ahead:");
        for (k, s) in moves.iter().rev().enumerate() {
            s.viz(k);
        }
    });
}
