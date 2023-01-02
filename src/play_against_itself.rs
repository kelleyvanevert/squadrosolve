use crate::game::{
    bully_heuristic, distance_heuristic, null_heuristic, steps_to_go_heuristic, State, COLORS,
    XULOS,
};

pub fn play_against_itself() {
    let mut num_wins = 0;
    let mut total_win_factor = 0.0;

    for i in 1.. {
        println!("Playing game {i}...");
        let (win_factor, final_state) = play_one_game(false);
        total_win_factor += win_factor as f64;
        if final_state.is_win() {
            num_wins += 1;
        }

        final_state.viz(0);
        println!("Total wins (for yellow): {}", num_wins);
        println!(
            "Avg win factor (for yellow): {}",
            total_win_factor / (i as f64)
        );
    }
}

fn play_one_game(debug: bool) -> (i32, State) {
    let lookahead = [8, 8];

    let mut state = State::initial();
    if debug {
        println!("\nInitial state:");
        state.viz(0);
    }

    while !state.completed() {
        if debug {
            println!(
                "\nComputing move {} for {} (looking {} steps ahead)...",
                state.depth + 1,
                COLORS[state.turn],
                lookahead[state.turn]
            );
        }
        let (moves, _) = state.minimax(lookahead[state.turn], state.turn == 0, |state| {
            if state.turn == 0 {
                bully_heuristic(&state)
            } else {
                distance_heuristic(&state)
            }

            // // Work together, try really hard for Yellow to win as best as possible
            // // ==
            // if state.turn == 0 {
            //     steps_to_go_heuristic(&state)
            // } else {
            //     // xuloser, no offence 🤣
            //     let num_stones_done = (1usize..=5)
            //         .filter(|&i| state.dir[XULOS][i] == 1 && state.at[XULOS][i] == 0)
            //         .count() as i32;

            //     num_stones_done
            // }
        });
        state = moves.last().unwrap().clone();
        if debug {
            state.viz(0);
        }
    }

    (state.win_factor(), state)
}
