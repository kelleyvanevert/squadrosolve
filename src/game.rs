use std::str::FromStr;

const YELLO: usize = 0;
const XULOS: usize = 1;

// color -> dir -> num
const STEP: [[[usize; 6]; 2]; 2] = [
    [[0, 3, 1, 2, 1, 3], [0, 1, 3, 2, 3, 1]],
    [[0, 1, 3, 2, 3, 1], [0, 3, 1, 2, 1, 3]],
];

const NUMS: [&'static str; 4] = ["0", "1", "2", "3"];
const COLORS: [&'static str; 2] = ["Yellow", "Xulos"];

const DEBUG: bool = false;

/*     X U L O S

    ┌  ─  1  3  2  3  1 ─ ┐
Y   |  . x1 x2 x3 x4 x5 . |
E   3 y1                . 1
L   1 y2                . 3
L   2 y3                . 2
O   1 y4                . 3
W   3 y5                . 1
    |  .  .  .  .  .  . . |
    └  ─  3  1  2  1  3 ─ ┘

*/

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    pub depth: usize,
    pub turn: usize,
    pub at: [[usize; 6]; 2],
    pub dir: [[usize; 6]; 2], // 0 going to, 1 going back
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

        for y in 1..=5 {
            let dir = if (1..=7).any(|x| grid[y + 1][x] == '>') {
                0
            } else {
                1
            };
            let step = (1..=7)
                .position(|x| grid[y + 1][x] == if dir == 0 { '>' } else { '<' })
                .unwrap();

            state.dir[YELLO][y] = dir;
            state.at[YELLO][y] = step;
        }

        for x in 1..=5 {
            let dir = if (1..=7).any(|y| grid[y][x + 1] == 'v') {
                0
            } else {
                1
            };
            let step = (1..=7)
                .position(|y| grid[y][x + 1] == if dir == 0 { 'v' } else { '^' })
                .unwrap();

            state.dir[XULOS][x] = dir;
            state.at[XULOS][x] = step;
        }

        Ok(state)
    }
}

impl State {
    pub fn initial() -> Self {
        State {
            depth: 0,
            turn: YELLO,
            at: [[0; 6]; 2],
            dir: [[0; 6]; 2],
        }
    }

    pub fn with_turn(mut self, turn: usize) -> Self {
        self.turn = turn;
        self
    }

    fn heuristic_for_color(&self, color: usize) -> i32 {
        (1..=5)
            .map(|i| {
                if self.dir[color][i] == 1 {
                    // going back
                    6 + 6 - self.at[color][i] as i32
                } else {
                    self.at[color][i] as i32
                }
            })
            .sum()
    }

    pub fn heuristic(&self) -> i32 {
        self.heuristic_for_color(YELLO) - self.heuristic_for_color(XULOS)
    }

    pub fn next(&self) -> Vec<(usize, Self)> {
        let mut possible = vec![];

        let color = self.turn;

        for i in 1..=5 {
            if DEBUG {
                println!("Moving stone {i} of {}", COLORS[color]);
            }

            if self.at[color][i] == 0 && self.dir[color][i] == 1 {
                // noop, this one's done
                if DEBUG {
                    println!("  - already done");
                }
                continue;
            }

            let mut s = self.clone();

            let j_curr = s.at[color][i];
            let dir = s.dir[color][i];
            let step_by = STEP[color][dir][i];
            let j_target = if dir == 0 {
                (j_curr + step_by).min(6)
            } else {
                j_curr.saturating_sub(step_by)
            };

            if DEBUG {
                println!(
                    "  at {} going {}, step by {}, target: {}",
                    j_curr, dir, step_by, j_target
                );
            }

            let mut j = if dir == 0 {
                j_curr + 1
            } else {
                j_curr.saturating_sub(1)
            };
            let mut hit = false;
            loop {
                if DEBUG {
                    println!("    {}", j);
                }

                if j > 0 && j < 6 && self.at[1 - color][j] == i {
                    if DEBUG {
                        println!("      hit {:?}", self.at[1 - color]);
                    }
                    hit = true;

                    // place back
                    if s.dir[1 - color][j] == 0 {
                        s.at[1 - color][j] = 0;
                    } else {
                        s.at[1 - color][j] = 6;
                    }
                }

                if !hit && j == j_target {
                    break;
                } else if j == 0 || j == 6 {
                    break;
                } else if hit && self.at[1 - color][j] != i {
                    break;
                }

                j = if dir == 0 { j + 1 } else { j.saturating_sub(1) };
            }

            s.at[color][i] = j;

            if s.dir[color][i] == 0 && s.at[color][i] == 6 {
                s.dir[color][i] = 1;
            }

            s.depth += 1;
            s.turn = 1 - s.turn;
            if DEBUG {
                s.viz(self.depth);
            }
            possible.push((i, s));
        }

        possible
    }

    pub fn completed(&self) -> bool {
        (1..=5).all(|i| self.dir[self.turn][i] == 1 && self.at[self.turn][i] == 0)
    }

    pub fn minimax(&self, depth: usize, maximize: bool) -> (Vec<State>, State) {
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
                .max_by_key(|s| s.1.heuristic());

            best_move_for_me.unwrap()
        } else {
            let best_move_for_them = self
                .next()
                .into_iter()
                .map(|(_, s)| {
                    let (mut moves, best_i_can_do) = s.minimax(depth - 1, !maximize);
                    moves.push(s);
                    (moves, best_i_can_do)
                })
                .min_by_key(|s| s.1.heuristic());

            best_move_for_them.unwrap()
        }
    }

    pub fn viz(&self, indent: usize) {
        let mut grid = vec![
            vec!["/", "-", "-", "-", "-", "-", "-", "-", "\\"],
            vec!["|", " ", ".", ".", ".", ".", ".", " ", "|"],
            vec!["|", ".", " ", " ", " ", " ", " ", ".", "|"],
            vec!["|", ".", " ", " ", " ", " ", " ", ".", "|"],
            vec!["|", ".", " ", " ", " ", " ", " ", ".", "|"],
            vec!["|", ".", " ", " ", " ", " ", " ", ".", "|"],
            vec!["|", ".", " ", " ", " ", " ", " ", ".", "|"],
            vec!["|", " ", ".", ".", ".", ".", ".", " ", "|"],
            vec!["\\", "-", "-", "-", "-", "-", "-", "-", "/"],
        ];

        for i in 1..=5 {
            // yellow n's
            grid[i + 1][0] = NUMS[STEP[YELLO][0][i]]; // to
            grid[i + 1][8] = NUMS[STEP[YELLO][1][i]]; // back

            // xulos n's
            grid[0][i + 1] = NUMS[STEP[XULOS][0][i]]; // to
            grid[8][i + 1] = NUMS[STEP[XULOS][1][i]]; // back

            // yellow
            let c = if self.dir[YELLO][i] == 0 { ">" } else { "<" };
            let x = self.at[YELLO][i];
            grid[i + 1][x + 1] = c;

            // xulos
            let c = if self.dir[XULOS][i] == 0 { "v" } else { "^" };
            let y = self.at[XULOS][i];
            grid[y + 1][i + 1] = c;
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
