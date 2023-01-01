use std::{fs, str::FromStr, time::Instant};

const YELLO: usize = 0;
const XULOS: usize = 1;

// color -> dir -> num
const STEP: [[[usize; 5]; 2]; 2] = [
    [[3, 1, 2, 1, 3], [1, 3, 2, 3, 1]],
    [[1, 3, 2, 3, 1], [3, 1, 2, 1, 3]],
];

const NUMS: [&'static str; 4] = ["0", "1", "2", "3"];

/*     X U L O S

    ┌ ─ 1 3 2 3 1 ─ ┐
Y   | . v v v v v . |
E   3 >           . 1
L   1 >           . 3
L   2 >           . 2
O   1 >           . 3
W   3 >           . 1
    | . . . . . . . |
    └ ─ 3 1 2 1 3 ─ ┘

*/

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    depth: usize,
    turn: usize,
    step: [[usize; 5]; 2],
    dir: [[usize; 5]; 2], // 0 going to, 1 going back
}

impl FromStr for State {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|line| {
                line.chars()
                    .enumerate()
                    .filter(|p| p.0 % 2 == 0)
                    .map(|p| p.1)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>();

        let mut state = State::initial();

        for y in 0..5 {
            let dir = if (1..=7).any(|x| grid[y + 2][x] == '>') {
                0
            } else {
                1
            };
            let mut step = (1..=7)
                .position(|x| grid[y + 2][x] == if dir == 0 { '>' } else { '<' })
                .unwrap();
            if dir == 1 {
                step = 6 - step;
            }
            state.dir[YELLO][y] = dir;
            state.step[YELLO][y] = step;
        }

        for x in 0..5 {
            let dir = if (1..=7).any(|y| grid[y][x + 2] == 'v') {
                0
            } else {
                1
            };
            let mut step = (1..=7)
                .position(|y| grid[y][x + 2] == if dir == 0 { 'v' } else { '^' })
                .unwrap();
            if dir == 1 {
                step = 6 - step;
            }
            state.dir[XULOS][x] = dir;
            state.step[XULOS][x] = step;
        }

        Ok(state)
    }
}

impl State {
    fn initial() -> Self {
        State {
            depth: 0,
            turn: YELLO,
            step: [[0; 5]; 2],
            dir: [[0; 5]; 2],
        }
    }

    fn heuristic_for_color(&self, color: usize) -> i32 {
        (0..5)
            .map(|i| {
                if self.dir[color][i] == 1 {
                    // going back
                    2 + self.step[color][i] as i32
                } else {
                    self.step[color][i] as i32
                }
            })
            .sum()
    }

    fn heuristic(&self) -> i32 {
        self.heuristic_for_color(YELLO) - self.heuristic_for_color(XULOS)
    }

    // fn xypos(&self, color: usize, i: usize) -> [usize; 2] {
    //     let mut pos = [0, 0]; // [x, y]
    //     if self.dir[color][i] == 0 {
    //         pos[color] = self.step[color][i];
    //     } else {
    //         pos[color] = 6 - self.step[color][i];
    //     }
    //     pos[1 - color] = i + 1;
    //     pos
    // }

    fn at(&self, color: usize, i: usize) -> usize {
        if self.dir[color][i - 1] == 0 {
            self.step[color][i - 1]
        } else {
            6 - self.step[color][i - 1]
        }
    }

    fn next(&self) -> Vec<(usize, Self)> {
        let mut possible = vec![];

        let color = self.turn;

        for i in 0..5 {
            // println!(
            //     "Moving stone {i} of {}",
            //     if color == 0 { "yellow" } else { "xulos" }
            // );

            if self.step[color][i] >= 6 && self.dir[color][i] >= 1 {
                // noop, this one's done
                continue;
            }

            let mut s = self.clone();

            let j_target = (s.step[color][i] + STEP[color][s.dir[color][i]][i]).min(6);

            // println!(
            //     "  at: {}, move by {}, target: {}",
            //     s.step[color][i], STEP[color][s.dir[color][i]][i], j_target
            // );

            let mut j = s.step[color][i] + 1;
            let mut hit = false;
            loop {
                // println!("    {}", j);

                // if self.at(1 - color, j) == i + 1 {
                if j < 5 && self.at(1 - color, j) == i + 1 {
                    // println!("      hit");
                    hit = true;
                    s.step[1 - color][j - 1] = 0; // place back to start (of dir)
                }

                if !hit && j == j_target {
                    break;
                } else if hit && (j == 6 || self.at(1 - color, j) != i + 1) {
                    break;
                } else if j == 6 {
                    break;
                }

                j += 1;
            }

            s.step[color][i] = j;

            if s.dir[color][i] == 0 && s.step[color][i] >= 6 {
                s.dir[color][i] = 1;
                s.step[color][i] = 0;
            }

            s.depth += 1;
            s.turn = 1 - s.turn;
            possible.push((i, s));
        }

        possible
    }

    fn completed(&self) -> bool {
        self.dir[self.turn].iter().all(|&d| d == 1) && self.step[self.turn].iter().all(|&s| s >= 6)
    }

    fn minimax(&self, depth: usize, maximize: bool) -> (Vec<State>, State) {
        if depth == 0 || self.completed() {
            (vec![], self.clone())
        } else if maximize {
            let best_move_for_me = self
                .next()
                .into_iter()
                .map(|(_, s)| {
                    let (mut moves, worst_they_can_do) = s.minimax(depth - 1, !maximize);
                    moves.push(s);
                    (moves, worst_they_can_do)
                })
                .max_by_key(|s| s.1.heuristic())
                .unwrap();

            best_move_for_me
        } else {
            let best_move_for_them = self
                .next()
                .into_iter()
                .map(|(_, s)| {
                    let (mut moves, best_i_can_do) = s.minimax(depth - 1, !maximize);
                    moves.push(s);
                    (moves, best_i_can_do)
                })
                .min_by_key(|s| s.1.heuristic())
                .unwrap();

            best_move_for_them
        }
    }

    fn viz(&self, indent: usize) {
        let mut grid = vec![
            vec!["/", "-", "-", "-", "-", "-", "-", "-", "\\"],
            vec!["|", ".", ".", ".", ".", ".", ".", ".", "|"],
            vec!["|", ".", " ", " ", " ", " ", " ", ".", "|"],
            vec!["|", ".", " ", " ", " ", " ", " ", ".", "|"],
            vec!["|", ".", " ", " ", " ", " ", " ", ".", "|"],
            vec!["|", ".", " ", " ", " ", " ", " ", ".", "|"],
            vec!["|", ".", " ", " ", " ", " ", " ", ".", "|"],
            vec!["|", ".", ".", ".", ".", ".", ".", ".", "|"],
            vec!["\\", "-", "-", "-", "-", "-", "-", "-", "/"],
        ];

        for i in 0..5 {
            // yellow n's
            grid[i + 2][0] = NUMS[STEP[YELLO][0][i]]; // to
            grid[i + 2][8] = NUMS[STEP[YELLO][1][i]]; // back

            // xulos n's
            grid[0][i + 2] = NUMS[STEP[XULOS][0][i]]; // to
            grid[8][i + 2] = NUMS[STEP[XULOS][1][i]]; // back

            // yellow
            let c = if self.dir[YELLO][i] == 0 { ">" } else { "<" };
            let x = if self.dir[YELLO][i] == 0 {
                self.step[YELLO][i]
            } else {
                6 - self.step[YELLO][i]
            };
            grid[i + 2][x + 1] = c;

            // xulos
            let c = if self.dir[XULOS][i] == 0 { "v" } else { "^" };
            let y = if self.dir[XULOS][i] == 0 {
                self.step[XULOS][i]
            } else {
                6 - self.step[XULOS][i]
            };
            grid[y + 1][i + 2] = c;
        }

        let s = grid
            .iter()
            .map(|row| format!("{}{}", vec![""; indent + 1].join("   "), row.join(" ")))
            .collect::<Vec<_>>()
            .join("\n");

        println!("\n{s}");
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heuristic().cmp(&other.heuristic())
    }
}

fn time<F>(mut f: F)
where
    F: FnMut(),
{
    let t0 = Instant::now();
    f();
    println!("  took {:?}", t0.elapsed());
}

fn main() {
    let (initial_state, read_from_file) = fs::read_to_string("./input.txt")
        .ok()
        .and_then(|s| (&s[0..]).parse().ok().map(|s| (s, true)))
        .unwrap_or((State::initial(), false));

    println!(
        "Initial state{}:",
        if read_from_file { " (FROM FILE)" } else { "" }
    );
    println!("========");
    initial_state.viz(0);

    // FOURTH ATTEMPT

    time(|| {
        let num_steps = 10;
        let (moves, last_state) = initial_state.minimax(num_steps, true);
        println!("Best move for me, looking {num_steps} steps ahead:");
        last_state.viz(0);
        println!("For moves:");
        for (k, s) in moves.iter().rev().enumerate() {
            s.viz(k);
        }
    });

    // // THIRD ATTEMPT

    // let best_move = initial_state
    //     .next()
    //     .into_iter()
    //     .max_by_key(|initial_state| {
    //         let mut consider = vec![initial_state.clone()];

    //         for i in 0..5 {
    //             println!();
    //             println!("Step {i}");

    //             println!("Adding all possible opponent's moves...");
    //             consider = consider.into_iter().flat_map(|s| s.next()).collect();
    //             println!("  now considering {}", consider.len());

    //             println!("Finding N best moves...");
    //             let mut beam = MinMaxHeap::new();
    //             for state in consider {
    //                 for next_state in state.next() {
    //                     let h = next_state.heuristic();

    //                     if beam.len() < MAX_BEAM_WIDTH {
    //                         beam.push(next_state);
    //                     } else if h < beam.peek_min().unwrap().heuristic() {
    //                         // not any good
    //                     } else {
    //                         beam.replace_min(next_state);
    //                     }
    //                 }
    //             }
    //             println!(
    //                 "  now considering {}, worst: {}, best: {}",
    //                 beam.len(),
    //                 beam.peek_min().unwrap().heuristic(),
    //                 beam.peek_max().unwrap().heuristic()
    //             );

    //             println!("Best:");
    //             beam.peek_max().unwrap().viz(0);

    //             consider = beam.into_vec();
    //         }
    //     });

    // println!();
    // println!("Best first move:");
    // if let Some(s) = best_move {
    //     s.viz(0);
    // } else {
    //     println!("NOT FOUND");
    // }

    // // SECOND ATTEMPT

    // let mut consider = vec![initial_state];
    // // let mut beam = MinMaxHeap::new();
    // // beam.push(initial_state);

    // for i in 0..5 {
    //     println!();
    //     println!("Step {i}");

    //     // find best moves
    //     println!("Finding best moves...");
    //     let mut beam = MinMaxHeap::new();
    //     for state in consider {
    //         for next_state in state.next() {
    //             let h = next_state.heuristic();

    //             if beam.len() < MAX_BEAM_WIDTH {
    //                 beam.push(next_state);
    //             } else if h < beam.peek_min().unwrap().heuristic() {
    //                 // not any good
    //             } else {
    //                 beam.replace_min(next_state);
    //             }
    //         }
    //     }
    //     println!(
    //         "  now considering {}, worst: {}, best: {}",
    //         beam.len(),
    //         beam.peek_min().unwrap().heuristic(),
    //         beam.peek_max().unwrap().heuristic()
    //     );

    //     println!("Best:");
    //     beam.peek_max().unwrap().viz(0);

    //     println!("Adding all possible opponent's moves...");
    //     consider = beam.into_iter().flat_map(|s| s.next()).collect();
    //     println!("  now considering {}", consider.len(),);
    // }

    // // FIRST ATTEMPT

    // let mut beam = MinMaxHeap::new();
    // beam.push(initial_state.heuristic());

    // let mut considering = vec![initial_state];
    // println!("Start with:");
    // considering[0].viz(0);

    // for i in 0.. {
    //     println!();
    //     println!("Step {i}");
    //     considering = min_step(&mut beam, considering);
    //     if let Some(state) = considering.iter().min_by_key(|s| s.heuristic()) {
    //         println!("Opponent's best move:");
    //         state.viz(0);
    //     }

    //     considering = max_step(&mut beam, considering);
    //     if let Some(state) = considering.iter().min_by_key(|s| s.heuristic()) {
    //         println!("My best move:");
    //         state.viz(0);
    //     }

    //     println!(
    //         "\nNow considering {}, beam width {} ({} thru {})",
    //         considering.len(),
    //         beam.len(),
    //         beam.peek_min().unwrap(),
    //         beam.peek_max().unwrap()
    //     );

    //     if considering.len() == 0 {
    //         break;
    //     }
    // }
}
