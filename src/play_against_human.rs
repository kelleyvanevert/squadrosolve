use crate::{
    game::{bully_heuristic, steps_to_go_heuristic, State},
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
        let (moves, _) = initial_state.minimax(num_steps, true, steps_to_go_heuristic);

        println!("Best move for me, looking {num_steps} steps ahead:");
        for (k, s) in moves.iter().rev().enumerate() {
            s.viz(k);
            if k == moves.len() - 1 {
                println!("{:?}", s);
            }
        }
        println!("Aka:");
        if let Some(s) = moves.iter().rev().next() {
            s.viz(0);
        }
    });
}
