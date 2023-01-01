use crate::{
    game::{distance_heuristic, null_heuristic, steps_to_go_heuristic, State, COLORS},
    util::time,
};

pub fn play_against_itself() {
    let lookahead = [8, 8];

    time(|| {
        let mut state = State::initial();
        println!("\nInitial state:");
        state.viz(0);

        while !state.completed() {
            println!(
                "\nComputing move {} for {} (looking {} steps ahead)...",
                state.depth + 1,
                COLORS[state.turn],
                lookahead[state.turn]
            );
            let (moves, _) = state.minimax(lookahead[state.turn], state.turn == 0, |state| {
                if state.turn == 0 {
                    null_heuristic(&state)
                } else {
                    distance_heuristic(&state)
                }
            });
            state = moves.last().unwrap().clone();
            state.viz(0);
        }
    });
}
